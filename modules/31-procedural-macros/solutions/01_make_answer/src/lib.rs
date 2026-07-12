//! 01 (0x) - функциональный макрос make_answer. Эталонное решение.
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn make_answer(_input: TokenStream) -> TokenStream {
    quote! {
        pub fn answer() -> u32 { 42 }
    }
    .into()
}
