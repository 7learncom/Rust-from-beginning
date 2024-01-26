fn main() {
    let mut its_ok = false;
    let age = 20;

    if age > 18 {
        its_ok = true
    }

    println!("is it safe for person ? {}", its_ok);

    let should_print = true;

    if should_print {
        println!("Its ok to print something")
    }

    // condition should be boolean
    // if 9 + 1 = 10 {}
}
