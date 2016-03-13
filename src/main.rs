// cargo run --features 'serde_type'
//
// テンプレートのライブリロード
// cargo run --features 'watch serde_type'

#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate mysql;
extern crate typemap;
extern crate plugin;
extern crate serde;
extern crate serde_json;
extern crate bson;
extern crate mongodb;

// #[macro_use(bson, doc)]

use std::error::Error;

use iron::prelude::*;
use router::Router;
#[cfg(feature = "watch")]
use std::sync::Arc;
#[cfg(feature = "watch")]
use hbs::Watchable;
use hbs::{HandlebarsEngine, DirectorySource};

mod db;
mod mongo;
mod background;
mod top;
mod hello;

fn main() {
    let mut router = Router::new();
    router.get("/", top::action);
    router.get("/hello", hello::action);

    let mut hbse = HandlebarsEngine::new2();
    hbse.add(Box::new(DirectorySource::new("./templates/", ".hbs")));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }

    let mut chain = Chain::new(router);
    chain_hbse(hbse, &mut chain);
    chain.link_around(db::DbMiddleware::new());

    chain.link_around(background::run());

    Iron::new(chain).http("localhost:1958").unwrap();
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
