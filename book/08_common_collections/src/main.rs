fn main() {
    let mut my_vec: Vec<i64> = Vec::new();

    my_vec.push(1);

    //  you can't add a string to a vector of integers
    // my_vec.push("a");

    // also it can infer
    let vector = vec![1, 2, 3];

    // vector can be dropped and this will persist
    // let third: &i64 = &vector[2];

    match vector.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    println!("{:?} {:?}", my_vec, vector);

    for i in &vector {
        println!("{}", i);
    }

    for i in &mut vec![100, 32, 57] {
        *i += 50;
        println!("{}", i);
    }
}
