const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;

    println!("The value of x is: {}", x);

    x = 6;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The pointer value of x is: {:p}", &x);

    println!("{}", THREE_HOURS_IN_SECONDS);
}
