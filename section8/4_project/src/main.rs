use std::f64::consts::PI;
trait CalculateArea {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

impl CalculateArea for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

fn print_custom_info<T: CalculateArea>(shape: &T, custom_info: impl Fn(f64) -> String) {
    let area = shape.area();
    let custom_info_result = custom_info(area);
    println!("Area: {:.2} , Custom Info: {}", area, custom_info_result)
}

fn main() {
    let circle = Circle::new(5.0);

    println!("Radius: {}", circle.radius);
    println!("Circumference: {:.2}", circle.circumference());

    println!("Area: {:.2}", circle.area());

    let area = |area| {
        if area > 50.0 {
            "Large".to_string()
        } else {
            "Small".to_string()
        }
    };
    
    print_custom_info(&circle, area)
}
