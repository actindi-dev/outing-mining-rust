// cargo run --features 'serde_type'
//
// テンプレートのライブリロード
// cargo run --features 'watch serde_type'

#![feature(custom_derive, plugin, stmt_expr_attributes)]
#![plugin(serde_macros)]

extern crate iron;
extern crate iron_session;
extern crate router;
extern crate handlebars;
extern crate handlebars_iron as hbs;
extern crate mysql;
extern crate typemap;
extern crate plugin;
extern crate serde;
extern crate serde_json;
#[macro_use(bson, doc)]
extern crate bson;
extern crate mongo_driver;
extern crate chrono;
extern crate urlencoded;
extern crate uuid;

use std::error::Error;

use iron::prelude::*;
use iron_session::{HashSessionStore, Sessions, TypeMapSession};
use router::Router;
#[cfg(feature = "watch")]
use std::sync::Arc;
#[cfg(feature = "watch")]
use hbs::Watchable;
use hbs::{HandlebarsEngine, DirectorySource};
use uuid::Uuid;

use summary::SummaryMiddleware;

mod db;
mod mongo;
mod tail_event;
mod view_helper;
mod user;
mod summary;
mod oauth2callback;
mod top;
mod watch_login;
mod hello;

fn main() {
    let mut router = Router::new();
    router.get("/", top::action);
    router.get("/oauth2callback", oauth2callback::action);
    router.get("/watch-login", watch_login::action);
    router.get("/hello", hello::action);

    let mut hbse = HandlebarsEngine::new2();
    hbse.add(Box::new(DirectorySource::new("./templates/", ".hbs")));
    hbse.registry.write().unwrap().register_helper("commify", Box::new(view_helper::commify));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("hbse.reload() {}", r.description());
    }

    let mut chain = Chain::new(router);
    chain_hbse(hbse, &mut chain);
    chain.link_around(db::DbMiddleware::new());
    let mongo_middleware = mongo::MongoMiddleware::new();
    let pool = mongo_middleware.pool.clone();
    chain.link_around(mongo_middleware);

    let store: HashSessionStore<TypeMapSession> = HashSessionStore::new();
    let uuid = Uuid::new_v4();  // 本来は固定
    println!("uuid {}", uuid);
    chain.around(Sessions::new(uuid.as_bytes().to_vec(), store));

    chain.link_around(SummaryMiddleware::new());

    tail_event::run(pool);

    Iron::new(chain).http("127.0.0.1:1958").unwrap();
}

#[cfg(feature = "watch")]
fn chain_hbse(hbse: HandlebarsEngine, chain: &mut Chain) {
    let hbse = Arc::new(hbse);
    hbse.watch("./templates/");
    chain.link_after(hbse);
}
#[cfg(not(feature = "watch"))]
fn chain_hbse(hbse: HandlebarsEngine, chain: &mut Chain) {
    chain.link_after(hbse);
}
