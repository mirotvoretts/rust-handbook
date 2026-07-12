//! 07 (3x) - derive Builder с опциональными Option<T>-полями.
//!
//! Расширь derive-макрос `#[derive(Builder)]` так, чтобы поля типа `Option<T>`
//! считались необязательными: их сеттер принимает `T` (а не `Option<T>`), а
//! `build()` не требует их задавать (по умолчанию `None`). Поля прочих типов
//! остаются обязательными.
//!
//! Подсказки:
//! - главная трудность - распознать в типе поля обёртку `Option<...>` и достать
//!   внутренний тип. Тип поля - это `syn::Type`; интересует вариант `Type::Path`,
//!   у последнего сегмента пути имя `Option`, а его `PathArguments::AngleBracketed`
//!   несёт `GenericArgument::Type` с внутренним типом;
//! - удобно вынести распознавание в функцию `option_inner(ty) -> Option<&Type>`;
//! - для опционального поля `build` кладёт `self.#id` как есть, для обязательного -
//!   через `ok_or_else`.
//!
//! Заглушка раскрывается в `compile_error!`, пока макрос не реализован.
//! Документация Type: <https://docs.rs/syn/latest/syn/enum.Type.html>.
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive_builder(_input: TokenStream) -> TokenStream {
    quote! { compile_error!("реализуй derive Builder (упражнение не решено)"); }.into()
}
