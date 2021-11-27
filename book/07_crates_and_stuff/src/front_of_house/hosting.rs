use super::super::utils::math;

pub fn add_to_waitlist(client: &str) {
    println!("Client {} added to the wailist.", client);
    seat_at_table();
}

pub fn seat_at_table() {
    let table_number = math::plus_one(1);
    println!("Table Number {} seatled.", table_number);
}
