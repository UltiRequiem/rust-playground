fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let six = plus_one(Some(5));
    let none = plus_one(None);

    println!("{}", six.unwrap());
    println!("{:?}", six);
    println!("{:?}", none);
    // println!("{:?}", none.unwrap());
}
