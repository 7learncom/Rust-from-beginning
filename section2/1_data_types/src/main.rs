fn main() {
    let bit8: i8 = 1;
    let bit16: i16 = 100;
    let floatnumber: f64 = 10000.0100;
    let t: bool = true;
    let c: char = 'a';

    println!("my number is {} , {}", bit8, bit16);
    println!("my float number is {}", floatnumber);
    println!("my boolean is : {}", t);
    println!("my character is : {}", c);

    let tup: (i8, f32) = (500, 89.90);
    let (x, _y) = tup;
    println!("my x is : {}", x);

    let my_array = [1, 2, 3, 4];
}
