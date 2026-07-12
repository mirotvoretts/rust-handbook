//! 04 (1x) - derive Getters. Эталонное решение.
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Getters)]
pub fn derive_getters(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(DataStruct { fields: Fields::Named(f), .. }) => &f.named,
        _ => {
            return syn::Error::new_spanned(&ast, "Getters: только именованные поля")
                .to_compile_error()
                .into()
        }
    };
    let getters = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        let method = format_ident!("get_{}", ident);
        quote! {
            pub fn #method(&self) -> &#ty { &self.#ident }
        }
    });
    quote! {
        impl #name {
            #( #getters )*
        }
    }
    .into()
}
