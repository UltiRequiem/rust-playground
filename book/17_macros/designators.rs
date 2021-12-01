macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

create_function!(foo);
create_function!(bar);

fn main() {
    foo();
    bar();

    print_result!(1u8 + 1);

    print_result!({
        let x = 10_999usize;

        x * x + 2000 * x - 10_000
    });
}
