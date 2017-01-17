#[derive(Deserialize, Debug)]
struct JsonData {
    access_token: String,
    token_type: String,
    expires_in: i32,
    id_token: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct Summary {
    pub name: &'static str,
    pub this_week: u32,
    pub last_week: u32,
    pub this_month: u32,
    pub last_month: u32,
    pub all: u32,
    pub week_percent: String,
    pub month_percent: String,
    start_end_sql: &'static str,
    all_sql: &'static str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
}

#[derive(Serialize, Debug)]
struct OfDate {
    date: String,
    success: std::collections::HashMap<String, usize>,
    failed: std::collections::HashMap<String, usize>,
}

#[derive(Serialize, Debug)]
struct IpCount {
    ip: String,
    count: usize,
}

#[derive(Serialize, Debug)]
struct TopData {
    date: String,
    success_count: usize,
    success_nip: usize,
    success_vec: Vec<IpCount>,
    failed_count: usize,
    failed_nip: usize,
    failed_vec: Vec<IpCount>,
}
