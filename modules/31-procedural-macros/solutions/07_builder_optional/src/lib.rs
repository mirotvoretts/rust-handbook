//! 07 (3x) - derive Builder с опциональными Option<T>-полями. Эталонное решение.
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Fields, GenericArgument, PathArguments, Type,
};

fn option_inner(ty: &Type) -> Option<&Type> {
    let Type::Path(tp) = ty else { return None };
    if tp.qself.is_some() {
        return None;
    }
    let seg = tp.path.segments.last()?;
    if seg.ident != "Option" {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &seg.arguments else {
        return None;
    };
    match args.args.first()? {
        GenericArgument::Type(inner) => Some(inner),
        _ => None,
    }
}

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

    let mut storage = Vec::new();
    let mut inits = Vec::new();
    let mut setters = Vec::new();
    let mut builds = Vec::new();

    for f in fields {
        let id = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        inits.push(quote! { #id: None });
        if let Some(inner) = option_inner(ty) {
            storage.push(quote! { #id: Option<#inner> });
            setters.push(quote! {
                pub fn #id(mut self, value: #inner) -> Self { self.#id = Some(value); self }
            });
            builds.push(quote! { #id: self.#id });
        } else {
            let msg = format!("поле {id} не задано");
            storage.push(quote! { #id: Option<#ty> });
            setters.push(quote! {
                pub fn #id(mut self, value: #ty) -> Self { self.#id = Some(value); self }
            });
            builds.push(quote! { #id: self.#id.ok_or_else(|| #msg.to_string())? });
        }
    }

    quote! {
        pub struct #builder_name {
            #( #storage, )*
        }
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name { #( #inits, )* }
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
