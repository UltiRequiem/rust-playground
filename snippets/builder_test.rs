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

    fn check_age(self) {
        match self.age {
            Some(age) => println!("{} is {}.", self.name, age),
            None => println!("Who knows how old {} is?", self.name),
        }
    }
}

fn main() {
    Person::new("Sally").age(27).check_age();
    Person::new("Bill").check_age();
    Person::new("Pedro").age(34).age(23).check_age();
}
