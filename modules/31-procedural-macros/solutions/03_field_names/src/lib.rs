//! 03 (1x) - derive FieldNames. Эталонное решение.
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(FieldNames)]
pub fn derive_field_names(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(DataStruct { fields: Fields::Named(f), .. }) => &f.named,
        _ => {
            return syn::Error::new_spanned(
                &ast,
                "FieldNames поддерживает только структуры с именованными полями",
            )
            .to_compile_error()
            .into()
        }
    };
    let names = fields.iter().map(|f| f.ident.as_ref().unwrap());
    quote! {
        impl #name {
            pub fn field_names() -> Vec<&'static str> {
                vec![ #( stringify!(#names) ),* ]
            }
        }
    }
    .into()
}
