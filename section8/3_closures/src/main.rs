fn main() {
    fn add_one_v1(x: i32) -> i32 {
        x + 1
    }

    let add_one_v2 = |x: i32| -> i32 { x + 1 };
    let add_one_v3 = |x: i32| x + 1;

    let mut capacity = "Hard disk capacity: 5000".to_string();
    {
        let mut my_closure = |c: char| capacity.push(c);
        let my_closure2 = |c: char| {
            println!("{:?}", capacity);
        };

        my_closure('G');
    }

    println!("{:?}", capacity);

    let u = 5;

    fn add_u(x: i32) -> i32 {
        x + u
    }
}
