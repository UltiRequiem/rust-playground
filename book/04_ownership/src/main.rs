fn main() {
    let s1 = String::from("hello");
    let mut s2 = s1;
    // let mut s2 = gives_ownership();

    // s1 is moved to s2, so s1 is invalid
    // println!("{}, world!", s1);

    println!("{}", s2);

    s2.push_str(", world!");

    {
        let s3 = s2.clone();
        println!("{}", s3);
        // println!("{}", s2);
    }

    println!("{}", s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    takes_and_gives_back(some_string)
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
