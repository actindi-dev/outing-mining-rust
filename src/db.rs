use std::env;
use std::sync::Arc;
use typemap::Key;
use iron::prelude::*;
use iron::{Handler, AroundMiddleware};
use mysql::{Pool, Opts};
use plugin::Extensible;

pub struct DbMiddleware {
    pool: Arc<Pool>,
}

struct DbHandler<H: Handler> { db: DbMiddleware, handler: H }

impl DbMiddleware {
    pub fn new() -> DbMiddleware {
        let url = match env::var("DB_URL") {
            Ok(val) => val,
            Err(_) => "mysql://root:@localhost:3307/outing_development".to_string(),
        };
        let pool = Pool::new(Opts::from_url(&url).unwrap()).unwrap();
        DbMiddleware { pool: Arc::new(pool) }
    }
}

impl Key for DbMiddleware { type Value = Arc<Pool>; }

impl<H: Handler> Handler for DbHandler<H> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        req.extensions_mut().insert::<DbMiddleware>(self.db.pool.clone());
        let res = self.handler.handle(req);
        res
    }
}

impl AroundMiddleware for DbMiddleware {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(DbHandler {
            db: self,
            handler: handler
        }) as Box<Handler>
    }
}

pub trait DbRequestExtension {
    fn db(&self) -> &Arc<Pool>;
}

impl<'a, 'b> DbRequestExtension for Request<'a, 'b> {
    fn db(&self) -> &Arc<Pool> {
        self.extensions().get::<DbMiddleware>().unwrap()
    }
}
