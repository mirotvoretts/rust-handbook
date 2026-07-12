//! 01 (0x) - функциональный процедурный макрос make_answer.
//!
//! Реализуй функциональный процедурный макрос `make_answer!()`, порождающий функцию
//! `pub fn answer() -> u32 { 42 }`. Входной поток токенов игнорируется.
//!
//! Требуемая сигнатура:
//!
//! ```ignore
//! #[proc_macro]
//! pub fn make_answer(input: TokenStream) -> TokenStream { ... }
//! ```
//!
//! Подсказки:
//! - тело функции собери макросом `quote! { ... }`, затем переведи результат в
//!   `TokenStream` вызовом `.into()`;
//! - входной аргумент не разбирается, свяжи его как `_input`.
//!
//! Заглушка раскрывается в `compile_error!`, пока макрос не реализован.
//! Документация quote: <https://docs.rs/quote>.
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn make_answer(_input: TokenStream) -> TokenStream {
    quote! { compile_error!("реализуй make_answer (упражнение не решено)"); }.into()
}
