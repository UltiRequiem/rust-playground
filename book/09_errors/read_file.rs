use std::{fs, io::Write, time::SystemTime};

fn main() {
    let mut hello_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open("hello.txt")
        .unwrap();

    match hello_file.write_all("Hello".as_bytes()) { Ok(_) => println!("Successfully wrote to the file!"),
        Err(error) => panic!("Problem writing to the file: {:?}", error),
    }
}
