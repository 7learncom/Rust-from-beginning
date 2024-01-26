fn main() {
    println!("10 + 3 = {}", 10 + 3);

    let ten = 10;
    let three = 3;

    let add = ten + three;
    println!("10 + 3 = {}", add);

    let sub = ten - three;
    println!("10 - 3 = {}", sub);

    let multi = ten * three;
    println!("10 * 3 = {}", multi);

    let divide: f64 = ten as f64 / three as f64;
    println!("10 / 3 = {}", divide);

    let remain = ten % three;
    println!("10 % 3 = {}", remain);
}
