struct Point<T, U> {
    x: T,
    y: T,
    z: U,
}

fn main() {
    let point = Point {
        x: 5000,
        y: 100,
        z: 89.0,
    };

    println!("Hello, world!");
}
