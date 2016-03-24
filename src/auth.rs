use iron::prelude::*;
use iron::{Handler, AroundMiddleware};
use iron::{headers, status};
use iron_session::TypeMapSession;
use plugin::Extensible;

use std::sync::{Arc, RwLock};
use typemap::ShareMap;


use user::User;

pub struct AuthMiddleware {
    except_paths: Vec<String>,
}

struct AuthHandler<H: Handler> {
    handler: H,
    except_paths: Vec<String>,
}

impl AuthMiddleware {
    pub fn new(except_paths: Vec<String>) -> AuthMiddleware {
        AuthMiddleware { except_paths: except_paths }
    }
}

fn session(request: &Request) -> Arc<RwLock<ShareMap>> {
    let lock = request.extensions().get::<TypeMapSession>().unwrap();
    lock.clone()
}

impl<H: Handler> Handler for AuthHandler<H> {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        if self.except_paths.contains(&request.url.path.join("/")) {
            return self.handler.handle(request);
        }
        let session = session(request);
        let res = if let Some(_) = session.read().unwrap().get::<User>() {
            self.handler.handle(request)
        } else {
            let mut response = Response::new();
            response.headers.set(headers::Location("/oauth2callback".to_string()));
            response.set_mut(status::Found);
            Ok(response)
        };
        res
    }
}

impl AroundMiddleware for AuthMiddleware {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(AuthHandler {
            handler: handler,
            except_paths: self.except_paths,
        }) as Box<Handler>
    }
}
