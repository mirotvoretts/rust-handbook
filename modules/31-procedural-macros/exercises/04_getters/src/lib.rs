//! 04 (1x) - derive-макрос Getters.
//!
//! Реализуй derive-макрос `#[derive(Getters)]`, который на каждое именованное поле
//! `field: Ty` порождает метод `pub fn get_field(&self) -> &Ty { &self.field }`.
//! Поддержать нужно только структуры с именованными полями.
//!
//! Подсказки:
//! - по полям структуры пройдись `fields.iter()`, из каждого возьми `ident` и `ty`;
//! - имя метода собери макросом `format_ident!("get_{}", ident)`;
//! - несколько методов вставляются в `impl` повторением `#( #getters )*`.
//!
//! Заглушка раскрывается в `compile_error!`, пока макрос не реализован.
//! Документация format_ident:
//! <https://docs.rs/quote/latest/quote/macro.format_ident.html>.
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Getters)]
pub fn derive_getters(_input: TokenStream) -> TokenStream {
    quote! { compile_error!("реализуй derive Getters (упражнение не решено)"); }.into()
}
