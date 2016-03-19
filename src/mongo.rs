use std::env;
use std::sync::Arc;

use iron::prelude::*;
use iron::{Handler, AroundMiddleware};
use mongo_driver::client::{Client, ClientPool,Uri};
use plugin::Extensible;
use typemap::Key;

#[derive(Debug)]
pub struct MongoMiddleware {
    pub pool: Arc<ClientPool>,
}

struct MongoHandler<H: Handler> {
    pool: Arc<ClientPool>,
    handler: H,
}


impl MongoMiddleware {
    pub fn new() -> MongoMiddleware {
        let uri = match env::var("MONGO_URI") {
            Ok(val) => val,
            Err(_) => "mongodb://localhost:27017".to_string(),
        };
        let uri = Uri::new(uri).unwrap();
        let pool = ClientPool::new(uri, None);
        MongoMiddleware { pool: Arc::new(pool) }
    }
}

impl Key for MongoMiddleware { type Value = Arc<ClientPool>; }

impl<H: Handler> Handler for MongoHandler<H> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        req.extensions_mut().insert::<MongoMiddleware>(self.pool.clone());
        self.handler.handle(req)
    }
}

impl AroundMiddleware for MongoMiddleware {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(MongoHandler {
            pool: self.pool.clone(),
            handler: handler,
        }) as Box<Handler>
    }
}

pub trait MongoRequestExtension {
    fn mongo(&self) -> Client;
}

impl<'a, 'b> MongoRequestExtension for Request<'a, 'b> {
    fn mongo(&self) -> Client {
        self.extensions().get::<MongoMiddleware>().unwrap().pop()
    }
}
