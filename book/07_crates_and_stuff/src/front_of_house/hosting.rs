use crate::utils::math::{plus_one, random_i8};

pub fn add_to_waitlist(client: &str) {
    println!("Client {} added to the waitlist.", client);
    seat_at_table();
}

pub fn seat_at_table() {
    println!("Table Number {} seated.", plus_one(random_i8()));
}
