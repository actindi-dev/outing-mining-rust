use std::collections::HashMap;
use serde_json::value;
use hbs::Template;

use iron::prelude::*;
use iron::status;
use urlencoded::UrlEncodedQuery;

pub fn action(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let mut data = HashMap::new();
    data.insert("title", value::to_value(&"oauth".to_string()));

    if let Ok(params) = request.get_ref::<UrlEncodedQuery>() {
        let code = &params["code"][0];
        data.insert("code", value::to_value(code));
    }

    response.set_mut(Template::new("oauth2callback", data)).set_mut(status::Ok);
    Ok(response)

}
