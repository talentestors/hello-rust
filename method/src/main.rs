mod my {
    #[derive(Debug)]
    pub struct Rectangle {
        width: u32,
        pub height: u32,
    }

    impl Rectangle {
        pub fn new(width: u32, height: u32) -> Self {
            Rectangle { width, height }
        }
        pub fn width(&self) -> u32 {
            return self.width;
        }
        pub fn height(&self) -> u32 {
            return self.height;
        }
        pub fn area(&self) -> u32 {
            self.width * self.height
        }
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
    // 这种方法往往用于初始化当前结构体的实例
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    fn calc_distance(c1: &Circle, c2: &Circle) -> f64 {
        let dx: f64 = c1.x - c2.x;
        let dy: f64 = c1.y - c2.y;
        (dx * dx + dy * dy).sqrt()
    }

    // Circle的方法，&self表示借用当前的Circle结构体
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

fn main() {
    let circle = Circle::new(0.0, 0.0, 5.0);
    println!("Area: {}", circle.area());
    let rect1 = my::Rectangle::new(30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("{}", rect1.width()); // OK
    println!("{}", rect1.height()); // OK
    // println!("{}", rect1.width); // Error - the visibility of field defaults to private
    println!("{}", rect1.height); // OK

    let rect1 = my::Rectangle::new(30, 50);
    let rect2 = my::Rectangle::new(10, 40);
    let rect3 = my::Rectangle::new(60, 45);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "c1 and c2 distance: {}",
        Circle::calc_distance(&Circle::new(0.0, 0.0, 5.0), &Circle::new(3.0, 4.0, 5.0))
    );
}
