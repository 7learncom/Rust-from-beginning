fn main() {
    let user1 = user_module::create_user(90);
    println!("my user id is {:?}", user1.id);
}

mod user_module {
    #[derive(Debug)]
    pub struct User {
        pub id: i8,
    }

    pub fn create_user(id: i8) -> User {
        if validate_id(id) {
            User { id }
        } else {
            panic!()
        }
    }

    fn validate_id(id: i8) -> bool {
        id > 0
    }
}
