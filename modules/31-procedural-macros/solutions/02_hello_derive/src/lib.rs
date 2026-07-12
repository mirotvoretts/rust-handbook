//! 02 (0x) - derive Hello. Эталонное решение.
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn derive_hello(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    quote! {
        impl #name {
            pub fn hello(&self) -> String {
                format!("Hello, {}!", stringify!(#name))
            }
        }
    }
    .into()
}
