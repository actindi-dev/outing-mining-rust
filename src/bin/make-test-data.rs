extern crate mongo_driver;
#[macro_use(bson, doc)]
extern crate bson;
extern crate rand;
extern crate chrono;

use mongo_driver::client::{ClientPool, Uri};
use bson::Bson;
use chrono::{DateTime, Duration, Local, Timelike, UTC};

fn main() {
    println!("テストデータを作ります。");
    let uri = Uri::new("mongodb://localhost:27017").unwrap();
    let pool = ClientPool::new(uri, None);
    let client = pool.pop();
    let mut logs_event = client.get_collection("outing", "logs.event");

    logs_event.drop().unwrap();

    for i in 0..30 {
        let time: DateTime<UTC> = UTC::now() - Duration::days(i);

        // ログイン成功
        let doc = doc! {
            "time" => time,
            "events" => [ { "login" => true } ],
            "ip" => (format!("10.10.10.{}", rand::random::<u8>()))
        };
        logs_event.insert(&doc, None).unwrap();

        // ログイン失敗
        let doc = doc! {
            "events" => [ { "login" => (Bson::Null) } ],
            "ip" => (format!("10.10.10.{}", rand::random::<u8>()))
        };
        logs_event.insert(&doc, None).unwrap();
    }

    let doc = doc!{ "dev" => "s" };
    println!("{:?}", logs_event.count(&doc, None));

    println!("{:?}", rand::random::<(f64, char)>());
    println!("{:?}", rand::random::<u32>());
}
