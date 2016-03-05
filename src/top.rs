use iron::prelude::*;
use iron::status;
use hbs::Template;
use mysql::from_row;
use db::DbRequestExtension;
use std::collections::BTreeMap;
use serde_json::value;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

#[derive(Serialize, Debug)]
struct Region {
    id: i32,
    name: String,
}

pub fn action(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let regions: Vec<Region> =
        request.db().prep_exec("select id, name from regions order by rand()", ()).unwrap().map(|row| {
            let (id, name) = from_row::<(i32, String)>(row.unwrap());
            Region { id: id, name: name }
        }).collect();

    let mut data = BTreeMap::new();
    data.insert("regions".to_string(), value::to_value(&regions));
    data.insert("foo".to_string(), value::to_value(&"ふー".to_string()));


    let client = Client::connect("localhost", 27017)
        .ok().expect("Failed to initialize standalone client.");
    let logs_event = client.db("outing").collection("logs.event");
    let mut cursor = logs_event.find(None, None).ok().expect("Failed to execute find.");
    cursor.next().map(|x| x.map(|doc| doc.get("pt").map(|pt| {
        println!("{}", pt);
        data.insert("pt".to_string(), value::to_value(pt));
    })));

    response.set_mut(Template::new("top", data)).set_mut(status::Ok);
    Ok(response)
}
