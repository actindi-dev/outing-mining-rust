use iron::typemap::Key;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
}

// impl User {
// pub fn current() -> Option<User> {
// None
// }
// }
//

impl Key for User {
    type Value = User;
}
