fn main() {
    let s1 = String::from("hello");

    println!("The length of '{}' is {}.", s1, calculate_length(&s1));
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
