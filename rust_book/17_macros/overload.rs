macro_rules! overloading {
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };

    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        );
    };
}

fn main() {
    overloading!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    overloading!(true; or false);
}
