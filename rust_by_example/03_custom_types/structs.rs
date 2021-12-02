struct Person {
    name: String,
    age: u8,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} is a {} years old human.", self.name, self.age)
    }
}

struct Unit;

struct Pair(i32, f32);

fn main() {
    let peter_parker = Person {
        name: "Peter".to_string(),
        age: 27,
    };

    println!("{}", peter_parker);

    let unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
