use iron::typemap::Key;

// impl User {
// pub fn current() -> Option<User> {
// None
// }
// }
//

impl Key for User {
    type Value = User;
}
