fn main() {
    for num in 0..=10 {
        println!("my number is: {}", num)
    }

    let a = [10, 20, 30, 40, 50];
    let b = 6; // we could not iterate over integer
    
    for value in a {
        println!("my value is: {}", value)
    }
}
