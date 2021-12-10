fn get_multiplier(factor: &i32) -> impl Fn(i32) -> i32 + '_ {
    move |x| x * factor
}

fn main() {
    let multiply_by_two = get_multiplier(&5);
    println!("{}", multiply_by_two(2));
}
