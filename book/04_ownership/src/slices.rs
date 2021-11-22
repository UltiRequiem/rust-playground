fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let hello = String::from("Hello world!");

    println!(
        "The first word of \"{full_message}\" is \"{first_word}\".",
        first_word = first_word(&hello),
        full_message = hello,
    );
}
