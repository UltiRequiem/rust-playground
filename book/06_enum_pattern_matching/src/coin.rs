enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => 25,
        }
    }
}

fn main() {
    let my_coin = Coin::Quarter(UsState::Alaska);

    println!("{}", my_coin.value_in_cents());
}
