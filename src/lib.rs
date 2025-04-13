use proc_macro::TokenStream;

mod auto_display;
mod timed;

use auto_display::process_auto_display;
use syn::{parse_macro_input, ItemFn};
use timed::process_timed;

#[proc_macro_derive(AutoDisplay, attributes(display))]
pub fn derive_auto_display(input: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = parse_macro_input!(input as syn::DeriveInput);
    process_auto_display(input).into()
}

#[proc_macro_attribute]
pub fn attribute_timed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: syn::ItemFn = parse_macro_input!(item as ItemFn);
    process_timed(input).into()
}
