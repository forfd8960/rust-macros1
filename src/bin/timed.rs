use std::{io, thread, time::Duration};

use rust_macros1::timed;

fn main() -> Result<(), io::Error> {
    my_function()?;
    Ok(())
}

#[timed]
fn my_function() -> Result<(), std::io::Error> {
    thread::sleep(Duration::from_millis(100));
    println!("hello my_function");
    Ok(())
}
