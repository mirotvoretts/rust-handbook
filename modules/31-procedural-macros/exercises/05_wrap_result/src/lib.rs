//! 05 (2x) - атрибутный макрос wrap_result.
//!
//! Реализуй атрибутный макрос `#[wrap_result]`, переписывающий функцию
//! `fn f(args) -> T { body }` в `fn f(args) -> Result<T, String> { Ok(body) }`.
//! Тип возврата оборачивается в `Result<_, String>`, тело - в `Ok(...)`. Функции
//! без явного возвращаемого значения дают `Result<(), String>`. Достаточно
//! поддержать простые функции без generics/where и без раннего `return`.
//!
//! Требуемая сигнатура:
//!
//! ```ignore
//! #[proc_macro_attribute]
//! pub fn wrap_result(attr: TokenStream, item: TokenStream) -> TokenStream { ... }
//! ```
//!
//! Подсказки:
//! - разбери элемент в `syn::ItemFn`; тебе нужны `vis`, `sig.ident`, `sig.inputs`,
//!   `sig.output` и `block`;
//! - `sig.output` - это `ReturnType`: вариант `Default` соответствует типу `()`,
//!   вариант `Type(_, ty)` несёт сам тип.
//!
//! Заглушка раскрывается в `compile_error!`, пока макрос не реализован.
//! Документация ItemFn: <https://docs.rs/syn/latest/syn/struct.ItemFn.html>.
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn wrap_result(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    quote! { compile_error!("реализуй wrap_result (упражнение не решено)"); }.into()
}
