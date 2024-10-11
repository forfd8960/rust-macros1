# rust-macros1

## Proc Macro attribute

* Dependencies

`proc-macro`: Provides the necessary types and attributes for creating procedural macros.
`quote`: Enables quasi-quoting, which allows constructing Rust code from within the macro.
`syn`: Parses Rust code into an Abstract Syntax Tree (AST) that the macro can manipulate.
`#[proc_macro_attribute]`: This attribute declares the function as a procedural macro that can be used as an attribute.

* Input Parsing:

`parse_macro_input!(item as ItemFn)`: Parses the input `TokenStream` (representing the function being attributed) into a `syn::ItemFn`, which represents a function definition in the AST.
Extracting Function Details:

`fn_name`: Gets the name of the function.
`fn_block`: Gets the code block of the function.
Constructing the Expanded Code:

`quote! { ... }`: **This is where the magic happens. It creates a new function with the same name (#fn_name).**

`let start = std::time::Instant::now();`: Records the start time before the original function's code is executed.

`let result = { #fn_block };`: Executes the original function's code block and stores the result. The curly braces ensure that the result is captured even if the original function doesn't return a value.
let duration = start.elapsed();: Calculates the elapsed time.

* Returning the Expanded Code:

`TokenStream::from(expanded)`: Converts the generated code back into a TokenStream that the compiler can understand.
