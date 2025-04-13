use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn process_auto_display(input: DeriveInput) -> TokenStream {
    // Parse the input tokens into a syntax tree
    // let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Get the fields of the struct
    let fields = match input.data {
        syn::Data::Struct(data_struct) => match data_struct.fields {
            syn::Fields::Named(fields) => fields.named,
            _ => panic!("AutoDisplay only supports structs with named fields"),
        },
        _ => panic!("AutoDisplay only supports structs"),
    };

    // Generate code for each field
    let field_displays = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! {
            write!(f, "{}: {}, ", stringify!(#field_name), self.#field_name)?;
        }
    });

    // Generate the expanded code
    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {{ ", stringify!(#name))?;
                #(#field_displays)*
                write!(f, "}}")
            }
        }
    };

    // Convert back to TokenStream and return
    TokenStream::from(expanded)
}
