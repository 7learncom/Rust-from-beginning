enum Message {
    Hello { id: i32 },
}

fn main() {
    let x = 3;

    match x {
        2..=6 => println!("from 2 to 6"),
        _ => println!("others"),
    }

    /////////////////////////////////////////////////////
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("found an id in range : {}", id_variable)
        }

        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }

        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
