enum Role {
    Admin,
    Support,
}

struct User {
    name: String,
    role: Role,
}

fn build_user(name: &str, role: Role) -> User {
    User {
        name: String::from(name),
        role,
    }
}
fn main() {
    let user = build_user("pouya", Role::Admin);

    match user.role {
        Role::Admin => println!("you can do this"),
        Role::Support => println!("stop here , you can not do this job"),
    }

    let user2 = build_user("nima", Role::Support);

    match user2.role {
        Role::Admin => println!("user2 you can do this"),
        Role::Support => println!("user2 stop here , you can not do this job"),
    }
}
