fn main() {
    let mut x = String::from("Hello"); // save in heap
    x.push_str(" World!");
    println!("x is {}", x);

    let y: String = x; // y borrowed x OR x move ownership to y

    println!("y is {}", y);

    {
        let scope = 6;

        println!("scop is valid ! and value is {}", scope)
    }

    println!("scop is not valid ! {}", scope)
}
