use std::env;
use std::io::Read;
use serde_json;
use serde_json::value;
use std::collections::BTreeMap;

use hbs::Template;
use iron::prelude::*;
use iron::{headers, status};
use urlencoded::UrlEncodedQuery;
use hyper;
use hyper::header::{ContentType, Headers, Authorization, Bearer};
use hyper_native_tls::NativeTlsClient;
use url::form_urlencoded;
use iron_sessionstorage::SessionRequestExt;

use user::User;

pub fn action(mut request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let mut data = BTreeMap::new();
    data.insert("title".to_string(), value::to_value(&"oauth"));

    data.insert("client_id".to_string(), value::to_value(client_id()));
    data.insert("redirect_uri".to_string(), value::to_value(redirect_uri()));

    if let Some(code) = get_code(&mut request) {
        // println!("code: {:?}", code);
        if let Some(access_token) = get_access_token(&code) {
            // println!("access_token: {:?}", access_token);
            if let Some(user) = get_user(&access_token) {
                // println!("user -> {:?}", user);
                if user.email.ends_with("@actindi.net") {
                    try!(request.session().set(user));
                    // TODO iron 0.3 なら RedirectRaw が使える...
                    response.headers.set(headers::Location("/".to_string()));
                    response.set_mut(status::Found);
                    return Ok(response);
                }
            }
        }
    }

    response.set_mut(Template::new("oauth2callback", data)).set_mut(status::Ok);
    Ok(response)
}

fn get_code(request: &mut Request) -> Option<String> {
    request.get_ref::<UrlEncodedQuery>()
        .ok()
        .and_then(|x| x.get("code"))
        .and_then(|x| x.get(0))
        .map(|x| x.clone())

}


fn client_id() -> String {
    env::var("OAUTH_CLIENT_ID").ok().unwrap()
}

fn client_secret() -> String {
    env::var("OAUTH_CLIENT_SECRET").ok().unwrap()
}

fn redirect_uri() -> String {
    env::var("OAUTH_REDIRECT_URI").ok().unwrap()
}

#[derive(Deserialize, Debug)]
struct JsonData {
    access_token: String,
    token_type: String,
    expires_in: i32,
    id_token: String,
}

fn get_access_token(code: &str) -> Option<String> {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = hyper::net::HttpsConnector::new(ssl);
    let client = hyper::Client::with_connector(connector);
    let req = form_urlencoded::Serializer::new(String::new())
        .append_pair("code", code)
        .append_pair("client_id", &client_id())
        .append_pair("client_secret", &client_secret())
        .append_pair("redirect_uri", &redirect_uri())
        .append_pair("grant_type", "authorization_code")
        .finish();
    let res = client.post("https://accounts.google.com/o/oauth2/token")
        .header(ContentType("application/x-www-form-urlencoded".parse().unwrap()))
        .body(&*req)
        .send();
    match res {
        Err(err) => {
            println!("err: {:?}", err);
            return None;
        }
        Ok(mut res) => {
            // println!("ok: {:?}", res);
            let mut json_str = String::new();
            res.read_to_string(&mut json_str).unwrap();
            // println!("json_str: {:?}", json_str);
            let json_data: Result<JsonData, _> = serde_json::from_str(&json_str);
            if let Ok(json_data) = json_data {
                // println!("JsonData: {:?}", json_data);
                return Some(json_data.access_token);
            }
            return None;
        }
    };
}

fn get_user(access_token: &String) -> Option<User> {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = hyper::net::HttpsConnector::new(ssl);
    let client = hyper::Client::with_connector(connector);

    let mut headers = Headers::new();
    headers.set(Authorization(Bearer { token: access_token.to_owned() }));
    let res = client.get("https://www.googleapis.com/oauth2/v2/userinfo")
        .headers(headers)
        .send();
    match res {
        Err(err) => {
            println!("err: {:?}", err);
            return None;
        }
        Ok(mut res) => {
            // println!("ok: {:?}", res);
            let mut json_str = String::new();
            res.read_to_string(&mut json_str).unwrap();
            // println!("json_str: {:?}", json_str);

            let user: User = serde_json::from_str(&json_str).unwrap();

            return Some(user);
        }
    };
}
