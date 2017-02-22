use std::sync::{Arc, Mutex};
use iron::prelude::*;
use iron::{Handler, AroundMiddleware};
use plugin::Extensible;
use typemap::Key;

use db::DbMiddleware;
use mysql::from_row;
use mysql::{Pool, Value};
use chrono::{Datelike, DateTime, Duration, Local, Timelike};

const REFRESH_PERIOD_SECONDS: i64 = 60 * 60;

#[derive(Serialize, Debug, Clone)]
pub struct Summary {
    pub name: &'static str,
    pub this_week: u32,
    pub last_week: u32,
    pub this_month: u32,
    pub last_month: u32,
    pub all: u32,
    pub week_percent: String,
    pub month_percent: String,
    start_end_sql: &'static str,
    all_sql: &'static str,
}

pub struct SummaryHolder {
    summaries: Vec<Summary>,
    updated_at: DateTime<Local>,
}

impl SummaryHolder {
    fn new() -> SummaryHolder {
        SummaryHolder {
            summaries: make_summaries(),
            updated_at: Local::now() - Duration::seconds(REFRESH_PERIOD_SECONDS),
        }
    }

    fn refresh(&mut self) {
        if Local::now() - self.updated_at > Duration::seconds(REFRESH_PERIOD_SECONDS) {
            self.updated_at = Local::now();
            for i in self.summaries.iter_mut() {
                i.refresh();
            }
        }
    }
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
            week_percent: "-".to_string(),
            month_percent: "-".to_string(),
            start_end_sql: start_end_sql,
            all_sql: all_sql,
        }
    }
    fn refresh(&mut self) {
        let pool = DbMiddleware::new().pool;
        let mut result = pool.prep_exec(self.all_sql, ()).unwrap();
        let row = result.next().unwrap().ok().unwrap();
        self.all = from_row::<u32>(row);

        let this_week_end = Local::now();
        let this_week_start = this_week_end - Duration::days(7);
        let last_week_end = this_week_start - Duration::seconds(1);
        let last_week_start = last_week_end - Duration::days(7);
        let this_month_end = this_week_end;
        let this_month_start = this_month_end - Duration::days(30);
        let last_month_end = this_month_start - Duration::seconds(1);
        let last_month_start = last_month_end - Duration::days(30);
        self.this_week = self.start_end_count(&pool, this_week_start, this_week_end);
        self.last_week = self.start_end_count(&pool, last_week_start, last_week_end);
        self.this_month = self.start_end_count(&pool, this_month_start, this_month_end);
        self.last_month = self.start_end_count(&pool, last_month_start, last_month_end);

        self.week_percent = if self.last_week == 0 {
            "-".to_string()
        } else {
            format!("{:.*}",
                    1,
                    self.this_week as f32 / self.last_week as f32 * 100f32)
        };
        self.month_percent = if self.last_month == 0 {
            "-".to_string()
        } else {
            format!("{:.*}",
                    1,
                    self.this_month as f32 / self.last_month as f32 * 100f32)
        };
    }

    fn start_end_count(&self,
                       pool: &Arc<Pool>,
                       start: DateTime<Local>,
                       end: DateTime<Local>)
                       -> u32 {
        let start = Value::Date(start.year() as u16,
                                start.month() as u8,
                                start.day() as u8,
                                start.hour() as u8,
                                start.minute() as u8,
                                start.second() as u8,
                                0);
        let end = Value::Date(end.year() as u16,
                              end.month() as u8,
                              end.day() as u8,
                              end.hour() as u8,
                              end.minute() as u8,
                              end.second() as u8,
                              0);
        let mut result = pool.prep_exec(self.start_end_sql, (start, end)).unwrap();
        let row = result.next().unwrap().ok().unwrap();
        from_row::<u32>(row)
    }
}


fn make_summaries() -> Vec<Summary> {
    vec![Summary::new("口こみ",
                      "select count(*) as count from experiences
             where created_at \
                       between ? and ?
             and publish = 1 and private = 0",
                      "select count(*) as count from experiences
             where publish = 1 \
                       and private = 0"),
         Summary::new("ありがとう",
                      "select count(*) as count from thanks
             where created_at \
                       between ? and ?",
                      "select count(*) as count from thanks"),
         Summary::new("行きたい！",
                      "select count(*) count from favorites
             where created_at \
                       between ? and ?",
                      "select count(*) as count from favorites"),
         Summary::new("ユーザ",
                      "select count(*) as count from users
             where created_at between \
                       ? and ?
             and type='Member' and activated_at is not null",
                      "select count(*) as count from users
             where type='Member' and \
                       activated_at is not null"),
         Summary::new("プレゼンタ",
                      "select count(*) as count from users
             where created_at between \
                       ? and ?
             and type='Provider' and activated_at is not null",
                      "select count(*) as count from users
             where type='Provider' \
                       and activated_at is not null")]
}


/// //////////////////////////////////////////////////////////////////////////
pub struct SummaryMiddleware {
    // TODO Mutex を RwLock に変える
    holder: Arc<Mutex<SummaryHolder>>,
}

impl SummaryMiddleware {
    pub fn new() -> SummaryMiddleware {
        SummaryMiddleware { holder: Arc::new(Mutex::new(SummaryHolder::new())) }
    }
}

impl Key for SummaryMiddleware {
    type Value = Arc<Mutex<SummaryHolder>>;
}

impl AroundMiddleware for SummaryMiddleware {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(MyHandler {
            holder: self.holder,
            handler: handler,
        }) as Box<Handler>
    }
}

struct MyHandler<H: Handler> {
    holder: Arc<Mutex<SummaryHolder>>,
    handler: H,
}

impl<H: Handler> Handler for MyHandler<H> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        req.extensions_mut().insert::<SummaryMiddleware>(self.holder.clone());
        let res = self.handler.handle(req);
        res
    }
}

pub trait SummaryExtension {
    fn summaries(&self) -> Vec<Summary>;
}

impl<'a, 'b> SummaryExtension for Request<'a, 'b> {
    fn summaries(&self) -> Vec<Summary> {
        let holder = self.extensions().get::<SummaryMiddleware>().unwrap();
        let mut holder = holder.lock().unwrap();
        holder.refresh();
        holder.summaries.clone()
    }
}
