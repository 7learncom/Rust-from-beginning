fn main() {
    let x: i32 = 100;
    let y: i32 = 200;

    println!("x is greater than y : {}", x > y);
    println!("x is less than y : {}", x < y);
    println!("x is unequal to y : {}", x != y);
    println!("x is greater/equal to y : {}", x >= y);
    println!("x is less/equal to y : {}", x <= y);
    println!("x is completely equal to y : {}", x == y);

    let c: i64 = 200;
    println!("c is greater than y : {}", c as i32 > y);
}
