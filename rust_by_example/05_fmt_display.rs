use std::fmt;

struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    println!(
        "The big range is {big} and the small is {small}.",
        small = MinMax(-3, 3),
        big = MinMax(-300, 300)
    );

    println!("{}", Point2D { x: 2.3, y: 9.2 });
}
