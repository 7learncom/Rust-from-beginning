fn main() {
    let condition = true;

    let number: i32 = if condition {
        5
    } else {
        6

        // we can not return another type
        // 'a'
    };

    println!("my number is : {}", number);
}
