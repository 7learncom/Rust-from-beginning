fn generate_vector<T>(start: T, end: T, step: T) -> Vec<T>
where
    T: std::ops::Add<Output = T> + std::cmp::PartialOrd + Copy,
{
    let mut result = Vec::new();
    let mut current = start;

    while current < end {
        result.push(current);
        current = current + step;
    }

    result
}

fn main() {
    // generate vector with i32 items start: 0 , end:10 , step: 2
    let int_vector = generate_vector(0, 10, 2);
    println!("Integer Vector: {:?}", int_vector);

    // generate vector with f64 items start: 1.0 , end:5.0 , step: 0.5
    let float_vector = generate_vector(1.0, 5.0, 0.5);
    println!("Float Vector: {:?}", float_vector);
}
