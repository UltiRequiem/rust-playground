fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;

    let an_integer = 5i32;

    let default_float = 3.0;
    let default_integer = 7;

    let mut inferred_type = 12;
    println!("inferred_type: {}", inferred_type);
    inferred_type = 4294967296i64;

    let mut mutable = 12;
    println!("mutable: {}", mutable);
    mutable = 21;

    println!(
        "{}, {}, {}, {}, {}, {}, {}, {}",
        logical,
        a_float,
        an_integer,
        default_float,
        default_integer,
        inferred_type,
        mutable,
        inferred_type
    );
}
