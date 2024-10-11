use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn timed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;

    let expand = quote! {
        fn #fn_name() -> Result<(), std::io::Error> {
            let start = std::time::Instant::now();
            let result = { #fn_block };
            let duration = start.elapsed();

            println!("function: {}, take: {:?}", stringify!(#fn_name), duration);
            result
        }
    };

    TokenStream::from(expand)
}
