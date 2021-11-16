enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

fn main() {
    inspect(WebEvent::KeyPress('x'));
    inspect(WebEvent::Paste("text".to_string()));
    inspect(WebEvent::Click { x: 20, y: 30 });
    inspect(WebEvent::PageLoad);
    inspect(WebEvent::PageUnload);
}
