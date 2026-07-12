//! 03 (1x) - derive-макрос FieldNames.
//!
//! Реализуй derive-макрос `#[derive(FieldNames)]`, добавляющий типу метод
//! `pub fn field_names() -> Vec<&'static str>` со списком имён полей структуры в
//! порядке объявления. Поддержать нужно только структуры с именованными полями;
//! на прочих входах выдать ошибку компиляции.
//!
//! Подсказки:
//! - именованные поля лежат в `Data::Struct(DataStruct { fields: Fields::Named(f), .. })`;
//! - имя поля - `field.ident`, строку получай через `stringify!`;
//! - для аккуратной ошибки используй `syn::Error::new_spanned(..).to_compile_error()`;
//! - список строк собирается повторением `#( ... ),*` внутри `vec![ ... ]`.
//!
//! Заглушка раскрывается в `compile_error!`, пока макрос не реализован.
//! Документация syn: <https://docs.rs/syn>.
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(FieldNames)]
pub fn derive_field_names(_input: TokenStream) -> TokenStream {
    quote! { compile_error!("реализуй derive FieldNames (упражнение не решено)"); }.into()
}
