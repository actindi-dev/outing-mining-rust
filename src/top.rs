use iron::prelude::*;
use iron::status;
use hbs::Template;
use std::collections::HashMap;
use serde_json::value;
use background::{Summary, SummaryExtension};

#[derive(Serialize, Debug)]
struct Region {
    id: i32,
    name: String,
}

pub fn action(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let mut data = HashMap::new();
    data.insert("title", value::to_value(&"トップ".to_string()));

    let ref vec: Vec<Summary> = *request.summaries().lock().unwrap();
    data.insert("summaries", value::to_value(vec));

    response.set_mut(Template::new("top", data)).set_mut(status::Ok);
    Ok(response)
}
