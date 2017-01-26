use iron::prelude::*;
use iron::status;
use hbs::Template;
use std::collections::HashMap;
use serde_json::value::{self, Value as Json, ToJson};
use summary::SummaryExtension;

pub fn action(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let mut data = HashMap::new();
    data.insert("title".to_string(), "トップ".to_json());
    data.insert("summaries".to_string(), request.summaries().to_json());

    response.set_mut(Template::new("top", data)).set_mut(status::Ok);
    Ok(response)
}
