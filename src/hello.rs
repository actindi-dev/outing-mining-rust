use iron::prelude::*;
use iron::status;
use hbs::Template;

pub fn action(_: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(Template::new("hello", ())).set_mut(status::Ok);
    Ok(response)
}
