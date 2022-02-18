fn multiplier(factor: &i32) -> impl Fn(i32) -> i32 + '_ {
    move |x| x * factor
}

fn main() {
    let multiply_by_two = multiplier(&5);

    for i in multiply_by_two(2)..multiply_by_two(8) {
        println!("{}", multiply_by_two(i));
    }
}
