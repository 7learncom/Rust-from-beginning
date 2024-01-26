#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new() -> Rectangle {
        Rectangle {
            height: 90,
            width: 90,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn area2(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn area1(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let area = area1(&rect1);

    let area2 = rect1.area();
    let area3 = rect1.area2();

    let square1 = Rectangle::square(40);
    let rect2 = Rectangle::new();
    
    println!("the area is : {}", area);
    println!("the area is : {}", area2);
    println!("the area is : {:?}", rect1);
}
