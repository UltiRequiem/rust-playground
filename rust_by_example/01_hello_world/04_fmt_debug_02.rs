// #[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!(
        // The "#" is for pretty printing
        "{:#?}",
        Person {
            name: "Zero",
            age: 15
        }
    );
}
