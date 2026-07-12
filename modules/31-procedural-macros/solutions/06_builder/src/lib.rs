//! 06 (2x) - derive Builder (все поля обязательны). Эталонное решение.
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let builder_name = format_ident!("{}Builder", name);
    let fields = match &ast.data {
        Data::Struct(DataStruct { fields: Fields::Named(f), .. }) => &f.named,
        _ => {
            return syn::Error::new_spanned(&ast, "Builder: только именованные поля")
                .to_compile_error()
                .into()
        }
    };
    let ids: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    let tys: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let setters = ids.iter().zip(tys.iter()).map(|(id, ty)| {
        quote! {
            pub fn #id(mut self, value: #ty) -> Self {
                self.#id = Some(value);
                self
            }
        }
    });
    let builds = ids.iter().map(|id| {
        let msg = format!("поле {id} не задано");
        quote! { #id: self.#id.ok_or_else(|| #msg.to_string())? }
    });
    quote! {
        pub struct #builder_name {
            #( #ids: Option<#tys>, )*
        }
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name { #( #ids: None, )* }
            }
        }
        impl #builder_name {
            #( #setters )*
            pub fn build(self) -> Result<#name, String> {
                Ok(#name { #( #builds, )* })
            }
        }
    }
    .into()
}
