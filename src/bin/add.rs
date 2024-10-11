macro_rules! add {
    ($a:expr, $b:expr) => {{
        $a + $b
    }};
}

macro_rules! add_as {
    ($($x:expr),*) => {
        {
            0
            $(+$x)*
        }
    };
}

macro_rules! add1 {
    ($x:expr) => {
        $x
    };
    ($x:expr, $y:expr) => {{
        $x + $y
    }};
    ($x:expr, $($y:tt)*) => {
        {
            $x + add1!($($y)*)
        }
    }
}

fn main() {
    println!("1+2 = {}", add!(1, 2));
    println!("add_as!(1) = {}", add_as!(1));
    println!("add_as!(1,2,3) = {}", add_as!(1, 2, 3));
    println!("add_as!() = {}", add_as!());

    println!("add1!(100) = {}", add1!(100));
    println!("add1!(100, 999) = {}", add1!(100, 999));
    println!("add1!(1,2,3) = {}", add1!(1, 2, 3));
}
