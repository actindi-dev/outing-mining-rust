use std::env;
use iron::prelude::*;
use mongodb::Client;
use mongodb::ThreadedClient;
use mongodb::error::Result;
use mongodb::topology::{TopologyDescription, TopologyType};
use mongodb::connstring;

#[derive(Debug, Clone,)]
pub struct Mongo {
    uri: String,
}

impl Mongo {
    pub fn new() -> Mongo {
        let uri = match env::var("MONGO_URI") {
            Ok(val) => val,
            Err(_) => "mongodb://localhost:27017".to_string(),
        };
        Mongo { uri: uri }
    }

    pub fn connect(&self) -> Result<Client> {
        // Client::with_uri(&self.uri); じゃつながらない
        let mut description = TopologyDescription::new();
        description.topology_type = TopologyType::Single;
        let config = connstring::parse(&self.uri).unwrap();
        Client::with_config(config, None, Some(description))
    }
}

pub trait MongoRequestExtension {
    fn mongo(&self) -> Result<Client>;
}

impl<'a, 'b> MongoRequestExtension for Request<'a, 'b> {
    fn mongo(&self) -> Result<Client> {
        Mongo::new().connect()
    }
}
