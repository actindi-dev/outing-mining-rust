use iron::prelude::*;
use iron::status;
use iron_session::TypeMapSession;
use hbs::Template;
use std::collections::HashMap;
use serde_json::value;
use summary::SummaryExtension;
use plugin::Extensible;

use user::User;

pub fn action(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let mut data = HashMap::new();
    data.insert("title", value::to_value(&"トップ".to_string()));
    data.insert("summaries", value::to_value(&request.summaries()));

    let lock = request.extensions().get::<TypeMapSession>().unwrap();
    let map = lock.read().unwrap();
    let user = map.get::<User>();
    data.insert("user", value::to_value(&user));

    response.set_mut(Template::new("top", data)).set_mut(status::Ok);
    Ok(response)
}
