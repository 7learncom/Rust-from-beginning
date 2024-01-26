use std::io;

fn main() {
    println!("Welcome to the guess number game!");

    let target_number = 56;
    let mut attempts = 0;

    loop {
        println!("Please enter your guess:");
        let mut guess = String::new();

        // read user input and add it to my guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // validation and parse string to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        attempts += 1;

        if guess == target_number {
            println!("well done , you guess right number with {} try", attempts);
            break;
        } else if guess < target_number {
            println!("too low")
        } else {
            println!("too high")
        }
    }
}
