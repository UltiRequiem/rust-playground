fn get_multiplier(factor: &i32) -> impl Fn(i32) -> i32 + '_ {
    move |x| x * factor
}

fn main() {
    let multiply_by_two = get_multiplier(&5);

    for i in 2..8 {
        println!("{}", multiply_by_two(i));
    }
}
