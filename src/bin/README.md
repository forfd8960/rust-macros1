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
