use iron::prelude::*;
use iron::status;

pub fn action(_request: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}
