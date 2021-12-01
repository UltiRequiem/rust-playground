use std::collections::HashMap;

const NAME: &str = "Diah";

pub fn main_one() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);
    println!("{}", scores.get("Blue").unwrap());
}

pub fn main_three() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);
}

pub fn main_two() {
    let mut my_vec: Vec<i64> = Vec::new();

    my_vec.push(1);

    //  you can't add a string to a vector of integers
    // my_vec.push("a");

    // also it can infer
    let vector = vec![1, 2, 3];

    // vector can be dropped and this will persist
    // let third: &i64 = &vector[2];

    match vector.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    println!("{:?} {:?}", my_vec, vector);

    for i in &vector {
        println!("{}", i);
    }

    for i in &mut vec![100, 32, 57] {
        *i += 50;
        println!("{}", i);
    }

    for char in NAME.chars() {
        println!("{}", char);
    }

    for char in NAME.bytes() {
        println!("{}", char);
    }
}
