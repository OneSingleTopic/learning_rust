#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(side_dim: u32) -> Rectangle {
        return Rectangle {
            width: side_dim,
            height: side_dim,
        };
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        (self.width > rectangle.width) && (self.height > rectangle.height)
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of rect1 is {}", rect1.area());

    let rect2 = Rectangle::square(10);
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
