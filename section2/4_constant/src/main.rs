const PI: f32 = 3.14;
fn main() {
    let x = 90;
    {
        let x = 90.90;
    }
    println!("my constant is {}", PI);
}
