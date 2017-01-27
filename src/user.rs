use iron_sessionstorage;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
}

impl iron_sessionstorage::Value for User {
    fn get_key() -> &'static str {
        "logged_in_user"
    }
    fn into_raw(self) -> String {
        self.id + " " + &self.email
    }
    fn from_raw(value: String) -> Option<Self> {
        if value.is_empty() {
            None
        } else {
            let mut id_email = value.split_whitespace();
            Some(User {
                id: id_email.next().unwrap().to_string(),
                email: id_email.next().unwrap().to_string(),
            })
        }
    }
}
