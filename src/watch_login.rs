use std::collections::HashMap;

use bson::Bson;
use chrono::{Date, Duration, Local};
use chrono::offset::utc::UTC;
use hbs::Template;
use iron::prelude::*;
use iron::status;
use serde_json::value;
use mongo_driver::client::Client;
use mongo::MongoRequestExtension;

pub fn action(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let mut data = HashMap::new();
    data.insert("title", value::to_value(&"ログイン監視".to_string()));

    let mongo = request.mongo();
    let login_data = watch_login_data(&mongo);
    data.insert("login_data", value::to_value(&login_data));

    data.insert("graph_data", value::to_value(&graph_data(&login_data)));
    data.insert("top_data", value::to_value(&top_data(&login_data)));

    response.set_mut(Template::new("watch_login", data)).set_mut(status::Ok);
    Ok(response)
}

fn top_data(vec: &Vec<::OfDate>) -> Vec<::TopData> {
    let mut results = Vec::new();
    for of_date in vec {
        let (sv, sc) = sort_and_total(&of_date.success);
        let (fv, fc) = sort_and_total(&of_date.failed);
        results.push(::TopData {
            date: of_date.date.clone(),
            success_count: sc,
            success_nip: of_date.success.len(),
            success_vec: sv,
            failed_count: fc,
            failed_nip: of_date.failed.len(),
            failed_vec: fv,
        });
    }
    results.reverse();
    results
}

fn sort_and_total(map: &HashMap<String, usize>) -> (Vec<::IpCount>, usize) {
    let mut vec = Vec::new();
    let mut total = 0;
    for (ip, count) in map {
        vec.push(::IpCount {
            ip: ip.clone(),
            count: *count,
        });
        total += *count;
    }
    vec.sort_by(|a, b| b.count.cmp(&a.count));
    vec.truncate(5);
    (vec, total)
}

fn graph_data(vec: &Vec<::OfDate>) -> String {
    let vec: Vec<String> = vec.iter().map(|i| format!("{}", i.failed.len())).collect();
    vec.join(", ")
}

fn watch_login_data(mongo: &Client) -> Vec<::OfDate> {
    let end = Local::today() - Duration::days(1);
    #[cfg(not(debug_assertions))]
    let mut date = end - Duration::days(29);
    #[cfg(debug_assertions)]
    let mut date = end - Duration::days(1);
    let mut vec = Vec::new();

    while date <= end {
        let (success, failed) = watch_log_per_date(&mongo, date);
        vec.push(::OfDate {
            date: date.format("%Y/%m/%d").to_string(),
            success: success,
            failed: failed,
        });
        date = date.succ();
    }
    vec
}

fn watch_log_per_date(mongo: &Client,
                      date: Date<Local>)
                      -> (HashMap<String, usize>, HashMap<String, usize>) {
    let end = date + Duration::days(1);

    let logs_event = mongo.get_collection("outing", "logs.event");
    let condition = doc! {
        "events" => {
            "$elemMatch" => {
                "login" => { "$exists" => true }
            }
        },
        "time" => {
            "$gte" => (date.and_hms(0, 0, 0).with_timezone(&UTC)),
            "$lt" => (end.and_hms(0, 0, 0).with_timezone(&UTC))
        }
    };
    let cursor = logs_event.find(&condition, None).unwrap();

    let mut success = HashMap::new();
    let mut failed = HashMap::new();
    let login_true = Bson::Document(doc! { "login" => true });
    for log in cursor.into_iter() {
        match log {
            Ok(log) => {
                match log.get_str("ip") {
                    Ok(ip) => {
                        let ip = ip.to_string();
                        match log.get_array("events") {
                            Ok(events) => {
                                // println!("events {:?} == {:?}", events, login_true);
                                if events.contains(&login_true) {
                                    if let Some(v) = success.get_mut(&ip) {
                                        *v += 1;
                                    }
                                    if !success.contains_key(&ip) {
                                        success.insert(ip.clone(), 1);
                                    }
                                } else {
                                    if let Some(v) = failed.get_mut(&ip) {
                                        *v += 1;
                                    }
                                    if !failed.contains_key(&ip) {
                                        failed.insert(ip.clone(), 1);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => panic!("Failed to get next from server! {}", e),
        }
    }
    (success, failed)
}
