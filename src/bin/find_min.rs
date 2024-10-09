/*
Macros can use + in the argument list to indicate that an argument may repeat at least once, or *, to indicate that the argument may repeat zero or more times.
*/

// surrounding the matcher with $(...),+ will match one or more expression, separated by commas.
macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => {
        std::cmp::min($x, find_min!($($y),+))
    }
}

/*
find_min!(1)  = 1
find_min!(100, 3, 9)  = 3
find_min!(5, 99, 66, -1, -1024)  = -1024
*/

fn main() {
    println!("find_min!(1)  = {}", find_min!(1));
    println!("find_min!(100, 3, 9)  = {}", find_min!(100, 3, 9));
    println!(
        "find_min!(5, 99, 66, -1, -1024)  = {}",
        find_min!(5, 99, 66, -1, -1024)
    );
}
