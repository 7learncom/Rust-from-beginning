fn main() {
    let x = 3;

    match x {
        1 => println!("One"),
        2 | 3 => println!("Two Or Three"),
        _ => println!("None of those values"),
    }

    let y = 200;

    match y {
        1..=100 => println!("its ok"),
        _ => println!("its not ok"),
    }

    let c = 'x';

    match c {
        'a'..='j' => println!("its good"),
        'k'..='z' => println!("its not good"),
        _ => println!("its not alphabet"),
    }
}
