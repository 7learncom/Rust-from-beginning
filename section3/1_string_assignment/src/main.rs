fn main() {
    let x = "hello".to_string();
    let y = String::from("hello");
    let z: &str = "hello";
    print!("{} {} {} ", x, y, z);
}
