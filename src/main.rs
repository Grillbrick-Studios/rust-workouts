fn main() {
// Rectangles
  let rect1 = Rectangle::new(30, 50);
  let rect2 = Rectangle::new(10, 40);
  let rect3 = Rectangle::new(60, 45);

  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn new(width: u32, height: u32) -> Self {
    Rectangle {
      width,
      height,
    }
  }

  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}
