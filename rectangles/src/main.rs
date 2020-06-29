#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let sq1 = Rectangle::square(10);
    println!("sq1 has area: {} square pixels", sq1.area());
    let rect1 = Rectangle {
        width: 30,
        height: 60,
    };
    println!("rect1 has area: {} square pixels", rect1.area());
    println!("Can rect1 hold sq1? {}", rect1.can_hold(&sq1));
}
