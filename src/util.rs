use std::sync::{Arc, RwLock};
use iron::prelude::*;
use iron_session::TypeMapSession;
use plugin::Extensible;
use typemap::ShareMap;

pub fn session(request: &Request) -> Arc<RwLock<ShareMap>> {
    let lock = request.extensions().get::<TypeMapSession>().unwrap();
    lock.clone()
}
