#![feature(try_blocks)]

fn main() {
    let r: Result<_, MyError> = try {
        do_step_1()?;
        do_step_2()?;
        do_step_3()?;
    };

    if let Err(_err) = r {
        println!("Failed to perform necessary steps");
    }
}

enum MyError {
    DoStep1Error,
    DoStep2Error,
    DoStep3Error,
}

fn do_step_1() -> Result<(), MyError> {
    println!("DoStep1");
    Ok(())
}

fn do_step_2() -> Result<(), MyError> {
    println!("DoStep2");
    Err(MyError::DoStep2Error)
}

fn do_step_3() -> Result<(), MyError> {
    println!("DoStep3");
    Ok(())
}
