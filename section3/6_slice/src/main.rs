fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let slice1: &[i32] = &a[2..5];

    println!("first value of slice: {}", slice1[0]);
    println!("second value of slice: {}", slice1[1]);
    println!("third value of slice: {}", slice1[2]);
    println!("last value of slice: {}", slice1[3]);
}
