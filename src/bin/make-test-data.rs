extern crate mongo_driver;
#[macro_use(bson, doc)]
extern crate bson;
extern crate rand;
extern crate chrono;

use mongo_driver::client::{ClientPool, Uri};
use bson::Bson;
use chrono::{DateTime, Duration, UTC};
use rand::distributions::{IndependentSample, Range};

fn main() {
    println!("テストデータを作ります。");
    let uri = Uri::new("mongodb://localhost:27017").unwrap();
    let pool = ClientPool::new(uri, None);
    let client = pool.pop();
    let mut logs_event = client.get_collection("outing", "logs.event");

    let _ = logs_event.drop();

    for i in 0..30 {
        let time: DateTime<UTC> = UTC::now() - Duration::days(i);

        // ログイン成功
        let between = Range::new(100, 200);
        let mut rng = rand::thread_rng();
        let max = between.ind_sample(&mut rng);
        for _ in 0..max {
            let doc = doc! {
                "time" => time,
                "events" => [ { "login" => true } ],
                "ip" => (format!("10.10.10.{}", rand::random::<u8>()))
            };
            logs_event.insert(&doc, None).unwrap();
        }

        // ログイン失敗
        let between = Range::new(10, 20);
        let max = between.ind_sample(&mut rng);
        for _ in 0..max {
            let doc = doc! {
                "time" => time,
                "events" => [ { "login" => (Bson::Null) } ],
                "ip" => (format!("10.10.10.{}", rand::random::<u8>()))
            };
            logs_event.insert(&doc, None).unwrap();
        }
    }

    let doc = doc!{};
    println!("{:?}", logs_event.count(&doc, None));
}
