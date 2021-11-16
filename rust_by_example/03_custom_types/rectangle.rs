struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        let Rectangle {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        } = *self;

        (x1 - x2) * (y1 - y2)
    }
}

fn main() {
    let rectangle = Rectangle {
        top_left: Point { x: 5.0, y: 5.0 },
        bottom_right: Point { x: 3.0, y: 4.0 },
    };

    println!("{}", rectangle.area());
}
