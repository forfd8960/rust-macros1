macro_rules! concat {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => {
        format!("{}{}", $x, concat!($($y),+))
    };
}

fn main() {
    println!("concat!(\"x\") = {}", concat!("x"));
    println!("concat!(\"x\", \"y\") = {}", concat!("x", "y"));
    println!("concat!(\"A\", \"B\", \"C\") = {}", concat!("A", "B", "C"));
}
