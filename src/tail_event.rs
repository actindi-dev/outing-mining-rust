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
        // let logs_event = client.get_collection("outing", "logs.event");
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

    fn insert_event_log(&self, logs_event: &Collection, log: Document) {
        match logs_event.insert(&log, None) {
            Ok(ok) => {
                println!("ok {:?}", ok);
            },
            Err(e) => {
                println!("Err {}", e);
                panic!("Err {}", e);
            }
        }
    }
}
/*
ok Some(ObjectId(ObjectId { id: [86, 234, 215, 201, 115, 44, 21, 55, 151, 212, 9, 165] }))
ok Some(ObjectId(ObjectId { id: [86, 234, 216, 121, 115, 44, 21, 55, 151, 212, 25, 45] }))
ok Some(ObjectId(ObjectId { id: [86, 234, 216, 242, 115, 44, 21, 55, 151, 212, 38, 4] }))

ok Some(ObjectId(ObjectId { id: [86, 234, 215, 201, 115, 44, 21, 55, 151, 212, 9, 165] }))
ok Some(ObjectId(ObjectId { id: [86, 234, 216, 121, 115, 44, 21, 55, 151, 212, 25, 45] }))
ok Some(ObjectId(ObjectId { id: [86, 234, 216, 242, 115, 44, 21, 55, 151, 212, 38, 4] }))
*/
