#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("Quit");
        }

        Message::Move { x, y } => {
            println!("Move x: {}, y: {}", x, y);
        }

        Message::Write(text) => {
            println!("Write: {}", text);
        }

        Message::ChangeColor(r, g, b) => {
            println!("ChangeColor r: {}, g: {}, b: {}", r, g, b);
        }
    }
}
