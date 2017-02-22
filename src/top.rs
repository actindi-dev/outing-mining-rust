use iron::prelude::*;
use iron::status;
use hbs::Template;
use std::collections::HashMap;
use summary::SummaryExtension;
use serde_json;

pub fn action(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let mut data = HashMap::new();
    data.insert("title".to_string(), serde_json::to_value("トップ"));
    data.insert("summaries".to_string(),
                serde_json::to_value(&request.summaries()));

    response.set_mut(Template::new("top", data)).set_mut(status::Ok);
    Ok(response)
}
