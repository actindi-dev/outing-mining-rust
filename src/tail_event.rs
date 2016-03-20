use std::sync::Arc;
use std::thread;
use bson::Document;
use mongo_driver::client::ClientPool;
use mongo_driver::collection::Collection;

struct TailProcess {
    pool: Arc<ClientPool>,
}

pub fn run(pool: Arc<ClientPool>) {
    thread::spawn(move || {
        TailProcess { pool: pool }.run();
    });
}

impl TailProcess {
    fn run(&self) {
        let client = self.pool.pop();

        let logs_app = client.get_collection("outing", "logs.app");
        #[cfg(not(debug_assertions))]
        let logs_event = client.get_collection("outing", "logs.event");
        #[cfg(debug_assertions)]
        let logs_event = client.get_collection("outing", "logs.event_test");

        let condition = doc! { "events" => { "$exists" => true } };
        let cursor = logs_app.tail(condition, None, None);
        for log in cursor.into_iter() {
            match log {
                Ok(log) => {
                    self.insert_event_log(&logs_event, log);
                },
                Err(e) => {
                    panic!("Err {}", e);
                },
            }
        }
    }

    // db.logs.event.find().sort({ time: -1 }).limit(1)
    fn insert_event_log(&self, logs_event: &Collection, log: Document) {
        let id = log.get("_id").unwrap().clone();
        let count = logs_event.count(&doc!{ "_id" => id }, None).unwrap();
        if count == 0 {
            match logs_event.insert(&log, None) {
                Ok(ok) => {
                    println!("ok {:?}", ok);
                },
                Err(e) => {
                    panic!("Err {}", e);
                }
            }
        }
    }
}
