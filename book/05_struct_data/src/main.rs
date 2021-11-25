use std::fmt;

struct Person {
    name: String,
    age: usize,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is {} years old.", self.name, self.age)
    }
}

impl Person {
    fn say_hi(&self) {
        println!("Hi, I'm {} and {} years old.", self.name, self.age);
    }
}

fn main() {
    let diah = &Person {
        name: String::from("Diah"),
        age: 15,
    };

    println!("{}", diah);

    diah.say_hi();
}
