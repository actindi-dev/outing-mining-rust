// cargo run
//
// テンプレートのライブリロード
// cargo run --features watch
extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;

use std::error::Error;

use iron::prelude::*;
use iron::status;
use router::Router;
use hbs::{Template, HandlebarsEngine, DirectorySource};
#[cfg(feature = "watch")]
use std::sync::Arc;
#[cfg(feature = "watch")]
use hbs::Watchable;

fn main() {

    let mut router = Router::new();
    router.get("/", |_: &mut Request| {
        Ok(Response::with((status::Ok, "トップ<br><a href='/hello'>hello</a>")))
    });
    router.get("/hello", hello_world);


    let mut hbse = HandlebarsEngine::new2();
    hbse.add(Box::new(DirectorySource::new("./templates/", ".hbs")));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }

    let mut chain = Chain::new(router);
    chain_hbse(hbse, &mut chain);

    Iron::new(chain).http("localhost:9000").unwrap();
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

fn hello_world(_: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(Template::new("hello", ())).set_mut(status::Ok);
    Ok(response)
}
