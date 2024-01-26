fn main() {
    let person: (&str, f64, i32) = ("james", 180.90, 23);

    let name = person.0;
    let height = person.1;
    let age = person.2;

    println!("{} is {} and has {} years", name, height, age)
}
