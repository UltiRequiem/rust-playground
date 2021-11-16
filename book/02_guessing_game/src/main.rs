use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..100);

    //  A mutable variable that is currently bound to a new, empty instance of a String.
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the user input.");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    println!("The secret number is: {}!", secret_number);
}
