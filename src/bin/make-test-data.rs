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

    let mut rng = rand::thread_rng();

    for i in 0..31 {
        let time: DateTime<UTC> = UTC::now() - Duration::days(i);

        // ログイン成功
        let max = Range::new(200, 300).ind_sample(&mut rng);
        for _ in 0..max {
            let doc = doc! {
                "time" => time,
                "events" => [ { "login" => true } ],
                "ip" => (format!("10.10.10.{}", rand::random::<u8>()))
            };
            logs_event.insert(&doc, None).unwrap();
        }

        // ログイン失敗
        let max = Range::new(50, 100).ind_sample(&mut rng);
        for _ in 0..max {
            let doc = doc! {
                "time" => time,
                "events" => [ { "login" => (Bson::Null) } ],
                "ip" => (format!("10.10.10.{}", rand::random::<u8>()))
            };
            logs_event.insert(&doc, None).unwrap();
        }

        // OAuth 失敗
        let max = Range::new(10, 30).ind_sample(&mut rng);
        for _ in 0..max {
            let doc = doc! {
                "time" => time,
                "events" => [ { "oauth" => false } ],
                "ip" => (format!("10.10.10.{}", rand::random::<u8>()))
            };
            logs_event.insert(&doc, None).unwrap();
        }

        // パスワードリセットリクエスト失敗
        let max = Range::new(20, 40).ind_sample(&mut rng);
        for _ in 0..max {
            let doc = doc! {
                "time" => time,
                "events" => [ { "password_reset_request" => false } ],
                "ip" => (format!("10.10.10.{}", rand::random::<u8>()))
            };
            logs_event.insert(&doc, None).unwrap();
        }

        // パスワードリセット失敗
        let max = Range::new(5, 25).ind_sample(&mut rng);
        for _ in 0..max {
            let doc = doc! {
                "time" => time,
                "events" => [ { "reset_password" => false } ],
                "ip" => (format!("10.10.10.{}", rand::random::<u8>()))
            };
            logs_event.insert(&doc, None).unwrap();
        }
    }

    let doc = doc!{};
    println!("{:?}", logs_event.count(&doc, None));
}
