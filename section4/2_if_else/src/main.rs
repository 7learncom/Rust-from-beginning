fn main() {
    let number = 1;

    if number % 4 == 0 {
        println!("by 4")
    } else if number % 3 == 0 {
        println!("by 3")
    } else if number % 2 == 0 {
        println!("by 2")
    } else {
        println!("not 4 or 3 or 2")
    }
}
