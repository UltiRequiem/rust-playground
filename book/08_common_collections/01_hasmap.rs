use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);

    println!("{:?}", scores);
    println!("{}", scores.get("Blue").unwrap());
}
