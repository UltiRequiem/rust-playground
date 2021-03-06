use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("GuessTheNumber V1");

    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Guess the number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("The secret number is: {}!", secret_number);
                break;
            }
        }
    }

    println!("Good job!")
}
