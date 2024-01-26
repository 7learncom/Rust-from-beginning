fn main() {
    let mut username = String::from("username");

    let newuser = &username;

    print_str(&username);

    add_str(&mut username);
    print_str2(&mut username);

    println!("username is {}", username);
}

fn print_str(str: &String) {
    println!("function scope {}", str)
}

fn print_str2(str: &mut String) {
    println!("function scope {}", str)
}

fn add_str(str: &mut String) {
    str.push_str(" add to str was success");
}
