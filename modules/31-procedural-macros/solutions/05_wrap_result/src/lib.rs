//! 05 (2x) - атрибут wrap_result. Эталонное решение.
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ReturnType};

#[proc_macro_attribute]
pub fn wrap_result(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let vis = &func.vis;
    let name = &func.sig.ident;
    let inputs = &func.sig.inputs;
    let block = &func.block;
    let ret_ty = match &func.sig.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => quote! { #ty },
    };
    quote! {
        #vis fn #name(#inputs) -> Result<#ret_ty, String> {
            Ok(#block)
        }
    }
    .into()
}
