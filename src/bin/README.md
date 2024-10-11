# macro1

## Expand create_function

```sh
➜  rust-macros1 git:(main) ✗ cargo expand --bin macro1
    Checking rust-macros1 v0.1.0 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn foo() {
    {
        ::std::io::_print(format_args!("You called {0:?}()\n", "foo"));
    };
}
fn bar() {
    {
        ::std::io::_print(format_args!("You called {0:?}()\n", "bar"));
    };
}
fn main() {
    foo();
    bar();
    {
        ::std::io::_print(format_args!("macro1\n"));
    };
}
```

## Expand print_result macro

```sh
➜  rust-macros1 git:(main) ✗ cargo expand --bin macro1
    Checking rust-macros1 v0.1.0 (/Users/zhiruchen/Documents/Code/github/forfd8960/rust-macros1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn foo() {
    {
        ::std::io::_print(format_args!("You called {0:?}()\n", "foo"));
    };
}
fn bar() {
    {
        ::std::io::_print(format_args!("You called {0:?}()\n", "bar"));
    };
}
fn main() {
    foo();
    bar();
    {
        ::std::io::_print(format_args!("{0:?} = {1:?}\n", "1u32 + 1023", 1u32 + 1023));
    };
    {
        ::std::io::_print(
            format_args!(
                "{0:?} = {1:?}\n",
                "{ let x = 123; let y = 890; x + y }",
                {
                    let x = 123;
                    let y = 890;
                    x + y
                },
            ),
        );
    };
    {
        ::std::io::_print(format_args!("macro1\n"));
    };
}
```

## overload

### expand

```sh
➜  rust-macros1 git:(main) cargo expand --bin overload
    Checking rust-macros1 v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    {
        ::std::io::_print(
            format_args!(
                "{0:?} and {1:?} is {2:?}\n",
                "0i32 + 9 == 9i32",
                "true",
                0i32 + 9 == 9i32 && true,
            ),
        );
    };
    {
        ::std::io::_print(
            format_args!("{0:?} or {1:?} is {2:?}\n", "true", "false", true || false),
        );
    };
}
```

```sh
"0i32 + 9 == 9i32" and "true" is true
"true" or "false" is true
```

## Find Main

```sh
➜  rust-macros1 git:(main) cargo expand --bin find_min
    Checking rust-macros1 v0.1.0 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    {
        ::std::io::_print(format_args!("find_min!(1)  = {0}\n", 1));
    };
    {
        ::std::io::_print(
            format_args!(
                "find_min!(100, 3, 9)  = {0}\n",
                std::cmp::min(100, std::cmp::min(3, 9)),
            ),
        );
    };
    {
        ::std::io::_print(
            format_args!(
                "find_min!(5, 99, 66, -1, -1024)  = {0}\n",
                std::cmp::min(
                    5,
                    std::cmp::min(99, std::cmp::min(66, std::cmp::min(-1, -1024))),
                ),
            ),
        );
    };
}
```

## Concat

```sh
➜  rust-macros1 git:(main) ✗ cargo expand --bin concat
    Checking rust-macros1 v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    {
        ::std::io::_print(format_args!("concat!(\"x\") = {0}\n", "x"));
    };
    {
        ::std::io::_print(
            format_args!(
                "concat!(\"x\", \"y\") = {0}\n",
                {
                    let res = ::alloc::fmt::format(format_args!("{0}{1}", "x", "y"));
                    res
                },
            ),
        );
    };
    {
        ::std::io::_print(
            format_args!(
                "concat!(\"A\", \"B\", \"C\") = {0}\n",
                {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "{0}{1}",
                            "A",
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}{1}", "B", "C"),
                                );
                                res
                            },
                        ),
                    );
                    res
                },
            ),
        );
    };
}
```

## Add

```sh
➜  rust-macros1 git:(main) ✗ cargo expand --bin add
    Checking rust-macros1 v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    {
        ::std::io::_print(format_args!("1+2 = {0}\n", { 1 + 2 }));
    };
    {
        ::std::io::_print(format_args!("add_as!(1) = {0}\n", { 0 + 1 }));
    };
    {
        ::std::io::_print(format_args!("add_as!(1,2,3) = {0}\n", { 0 + 1 + 2 + 3 }));
    };
    {
        ::std::io::_print(format_args!("add_as!() = {0}\n", { 0 }));
    };
    {
        ::std::io::_print(format_args!("add1!(1,2,3) = {0}\n", { 1 + { 2 + 3 } }));
    };
}
```

## Timed Macro

```sh
➜  rust-macros1 git:(main) ✗ cargo expand --bin timed
    Checking rust-macros1 v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::{io, thread, time::Duration};
use rust_macros1::timed;
fn main() -> Result<(), io::Error> {
    my_function()?;
    Ok(())
}
fn my_function() -> Result<(), std::io::Error> {
    let start = std::time::Instant::now();
    let result = {
        {
            thread::sleep(Duration::from_millis(100));
            {
                ::std::io::_print(format_args!("hello my_function\n"));
            };
            Ok(())
        }
    };
    let duration = start.elapsed();
    {
        ::std::io::_print(
            format_args!("function: {0}, take: {1:?}\n", "my_function", duration),
        );
    };
    result
}
```
