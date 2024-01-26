mod restaurant;
use restaurant::{cooking, welcome_message, Sample};

fn main() {
    welcome_message();
    cooking::send_food();
    let sample1 = Sample {};
}
