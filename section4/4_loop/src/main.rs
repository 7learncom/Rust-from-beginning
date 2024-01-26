fn main() {
    let mut num: usize = 0;
    loop {
        num = num + 1;

        if num == 10 {
            break;
        }
    }

    println!("{}", num);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 20 {
            break counter;
        }
    };

    println!("{}", result)
}
