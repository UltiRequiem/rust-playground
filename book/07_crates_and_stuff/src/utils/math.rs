use rand::Rng;

/// Adds one to the given number.
pub fn plus_one(x: i8) -> i8 {
    x + 1
}

pub fn random_i8() -> i8 {
    rand::thread_rng().gen_range(1..100)
}
