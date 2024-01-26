struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn build_user(email: String, username: &str) -> User {
    // email validation
    // username is unique
    // ...

    User {
        active: true,
        sign_in_count: 1,
        email: email,
        username: String::from(username),
    }
}
fn main() {
    // check email
    // check username if unique
    // ...
    let mut user1 = User {
        email: String::from("x@gmail.com"),
        active: true,
        username: String::from("sam1993"),
        sign_in_count: 1,
    };

    user1.sign_in_count += 1;
    println!("email is {}", user1.email);
    println!("sign_in_count is {}", user1.sign_in_count);

    let user2 = build_user(user1.email, "user2username");
    println!("user2 email is {}", user2.email);
    println!("user2 username is {}", user2.username);
}
