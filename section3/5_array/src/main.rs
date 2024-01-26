fn main() {
    // first way
    let a: [i32; 4] = [8; 4];
    let first_index = a[0];
    let second_index = a[1];

    println!("{} , {}", first_index, second_index);

    // second way
    let mut b = [1, 2, 3, 4, 5, 7];
    let last_index = b[5];
    println!("{}", last_index);

    b[5] = 6;
    println!("{}", b[5]);

    // third way
    let c: [f64; 3] = [1.0, 2.0, 3.0];
    println!("{}", c[1]);
}
