struct Rectangle {
    height: usize,
    width: usize,
}

impl Rectangle {
    // Static Methods, yey
    fn new(height: usize, width: usize) -> Self {
        Self { height, width }
    }

    fn area(&self) -> usize {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle::new(50, 60);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let rect_two = &Rectangle { height: 23, ..rect };

    println!(
        "The area of the second rectangle is {} square pixels.",
        rect_two.area()
    );
}
