fn main() {
    let mut v0: Vec<i32> = Vec::new();
    v0.push(1);
    v0.push(2);
    v0.push(3);
    for i in v0 {
        println!("{}", i)
    }

    let v1 = vec![1, 2, 3];
    let first_element: &i32 = &v1[0];
    println!("{}", first_element);

    let v2: Vec<f64> = vec![1.2; 10];
    let second_element = v2.get(20);
    println!("{:?}", second_element);

    {
        let v3 = vec![0, 1, 2, 3];
    }
    // println!("{}", v3[0]) // should not work
}
