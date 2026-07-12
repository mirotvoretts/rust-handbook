//! 02 (0x) - derive-макрос Hello.
//!
//! Реализуй derive-макрос `#[derive(Hello)]`, добавляющий типу метод
//! `pub fn hello(&self) -> String`, который возвращает строку `"Hello, <ИмяТипа>!"`.
//!
//! Требуемая сигнатура:
//!
//! ```ignore
//! #[proc_macro_derive(Hello)]
//! pub fn derive_hello(input: TokenStream) -> TokenStream { ... }
//! ```
//!
//! Подсказки:
//! - разбери вход в `syn::DeriveInput` через `parse_macro_input!`;
//! - имя типа лежит в `ast.ident`;
//! - строку с именем типа получи через `stringify!(#name)` внутри `quote!`.
//!
//! Заглушка раскрывается в `compile_error!`, пока макрос не реализован.
//! Документация syn: <https://docs.rs/syn>.
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Hello)]
pub fn derive_hello(_input: TokenStream) -> TokenStream {
    quote! { compile_error!("реализуй derive Hello (упражнение не решено)"); }.into()
}
