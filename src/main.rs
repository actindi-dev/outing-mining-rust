// cargo run --features 'serde_type'
//
// テンプレートのライブリロード
// cargo run --features 'watch serde_type'
extern crate iron;
extern crate iron_sessionstorage;
#[macro_use]
extern crate router;
extern crate handlebars;
extern crate handlebars_iron as hbs;
extern crate mysql;
extern crate typemap;
extern crate plugin;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use(bson, doc)]
extern crate bson;
extern crate mongo_driver;
extern crate chrono;
extern crate urlencoded;
extern crate uuid;
extern crate hyper;
extern crate hyper_native_tls;
extern crate url;

use std::error::Error;

use iron::prelude::*;
#[cfg(feature = "watch")]
use std::sync::Arc;
#[cfg(feature = "watch")]
use hbs::Watchable;
use hbs::{HandlebarsEngine, DirectorySource};
use uuid::Uuid;
use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;

mod db;
mod mongo;
mod tail_event;
mod view_helper;
mod user;
mod summary;
mod auth;
mod oauth2callback;
mod top;
mod watch_login;
mod hello;

fn main() {
    let router = router!(
        top: get "/" => top::action,
        oauth2callback: get "/oauth2callback" => oauth2callback::action,
        watch_login: get "/watch-login" => watch_login::action,
        hello: get "/hello" => hello::action,
    );

    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./templates/", ".hbs")));
    hbse.handlebars_mut().register_helper("commify", Box::new(view_helper::commify));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("hbse.reload() {}", r.description());
    }

    let mut chain = Chain::new(router);
    chain.link_around(auth::AuthMiddleware::new(vec!["oauth2callback".to_string()]));
    chain_hbse(hbse, &mut chain);
    chain.link_around(db::DbMiddleware::new());
    let mongo_middleware = mongo::MongoMiddleware::new();
    let pool = mongo_middleware.pool.clone();
    chain.link_around(mongo_middleware);

    let uuid = Uuid::new_v4().as_bytes().to_vec();  // 本来は固定
    chain.link_around(SessionStorage::new(SignedCookieBackend::new(uuid)));

    chain.link_around(summary::SummaryMiddleware::new());

    tail_event::run(pool);

    Iron::new(chain).http("rust:1958").unwrap();
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
