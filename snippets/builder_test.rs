struct Person<'a> {
    name: &'a str,
    age: Option<u8>,
}

impl<'a> Person<'a> {
    fn new(name: &'a str) -> Person<'a> {
        Person { name, age: None }
    }

    fn age(self, age: u8) -> Self {
        Person {
            age: Some(age),
            ..self
        }
    }
}

fn check_age(arg: Person) {
    match arg.age {
        Some(age) => println!("{} is {}.", arg.name, age),
        None => println!("Who knows how old {} is?", arg.name),
    }
}

fn main() {
    check_age(Person::new("Sally").age(27));
    check_age(Person::new("Bill"));
    check_age(Person::new("Pedro").age(34).age(23));
}
