use std::sync::{Arc, Mutex};
use iron::prelude::*;
use iron::{Handler, AroundMiddleware};
use plugin::Extensible;
use typemap::Key;

use db::DbMiddleware;
use mysql::from_row;

#[derive(Serialize, Debug)]
pub struct Summary {
    pub name: &'static str,
    pub this_week: i32,
    pub last_week: i32,
    pub this_month: i32,
    pub last_month: i32,
    pub all: i32,
    start_end_sql: &'static str,
    all_sql: &'static str,
}


pub fn run() -> SummaryMiddleware {
    let x = SummaryMiddleware::new();

    for i in x.summaries.lock().unwrap().iter_mut() {
        i.refresh();
    }

    x
}

impl Summary {
    pub fn new(name: &'static str, start_end_sql: &'static str, all_sql: &'static str) -> Summary {
        Summary {
            name: name,
            this_week: 0,
            last_week: 0,
            this_month: 0,
            last_month: 0,
            all: 0,
            start_end_sql: start_end_sql,
            all_sql: all_sql,
        }
    }
    fn refresh(&mut self) {
        let pool = DbMiddleware::new().pool;
        let mut result = pool.prep_exec(self.all_sql, ()).unwrap();
        let row = result.next().unwrap().ok().unwrap();
        self.all = from_row::<i32>(row);
    }
}


fn make_summaries() -> Vec<Summary> {
    vec!(
        Summary::new(
            "口こみ",
            "select count(*) as count from experiences
             where created_at between :start and :end
             and publish = 1 and private = 0",
            "select count(*) as count from experiences
             where publish = 1 and private = 0"),
        Summary::new(
            "ありがとう",
            "select count(*) as count from thanks
             where created_at between :start and :end",
            "select count(*) as count from thanks"),
        Summary::new(
            "行きたい！",
            "select count(*) count from favorites
             where created_at between :start and :end",
            "select count(*) as count from favorites"),
        Summary::new(
            "ユーザ",
            "select count(*) as count from users
             where created_at between :start and :end
             and type='Member' and activated_at is not null",
            "select count(*) as count from users
             where type='Member' and activated_at is not null"),
        Summary::new(
            "プレゼンタ",
            "select count(*) as count from users
             where created_at between :start and :end
             and type='Provider' and activated_at is not null",
            "select count(*) as count from users
             where type='Provider' and activated_at is not null") ,
    )
}


/////////////////////////////////////////////////////////////////////////////
pub struct SummaryMiddleware {
    summaries: Arc<Mutex<Vec<Summary>>>,
}

impl SummaryMiddleware {
    pub fn new() -> SummaryMiddleware {
        SummaryMiddleware { summaries: Arc::new(Mutex::new(make_summaries())) }
    }
}

impl Key for SummaryMiddleware {
    type Value = Arc<Mutex<Vec<Summary>>>;
}

impl AroundMiddleware for SummaryMiddleware {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(MyHandler {
            summaries: self.summaries,
            handler: handler,
        }) as Box<Handler>
    }
}

struct MyHandler<H: Handler> {
    summaries: Arc<Mutex<Vec<Summary>>>,
    handler: H,
}

impl<H: Handler> Handler for MyHandler<H> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        req.extensions_mut().insert::<SummaryMiddleware>(self.summaries.clone());
        let res = self.handler.handle(req);
        res
    }
}

pub trait SummaryExtension {
    fn summaries(&self) -> &Arc<Mutex<Vec<Summary>>>;
}

impl<'a, 'b> SummaryExtension for Request<'a, 'b> {
    fn summaries(&self) -> &Arc<Mutex<Vec<Summary>>> {
        self.extensions().get::<SummaryMiddleware>().unwrap()
    }
}
