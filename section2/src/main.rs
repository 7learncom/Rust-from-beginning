const FAHRENHEIT_FREEZING_POINT: i32 = 32;
const FAHRENHEIT_TO_CELSIUS_RATIO: f64 = 5.0 / 9.0;

fn main() {
    let fahrenheit_temp: i32 = 68;
    println!("Fahrenheit: {}", fahrenheit_temp);

    let celsius_temp = convert_to_celsius(fahrenheit_temp);
    println!("Celsius: {}", celsius_temp);
}

fn convert_to_celsius(fahrenheit: i32) -> f64 {
    let celsius = (fahrenheit - FAHRENHEIT_FREEZING_POINT) as f64 * FAHRENHEIT_TO_CELSIUS_RATIO;
    return celsius;
}
