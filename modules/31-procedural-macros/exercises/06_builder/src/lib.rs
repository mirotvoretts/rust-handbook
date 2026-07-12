//! 06 (2x) - derive-макрос Builder (все поля обязательны).
//!
//! Реализуй derive-макрос `#[derive(Builder)]` для структуры с именованными полями.
//! Он должен порождать тип `<Name>Builder` с полями `Option<Ty>`, метод
//! `Name::builder()`, сеттеры `fn field(self, value: Ty) -> Self` и метод
//! `fn build(self) -> Result<Name, String>`, возвращающий `Err`, если хоть одно
//! поле не задано.
//!
//! Подсказки:
//! - имя билдера собери через `format_ident!("{}Builder", name)`;
//! - собери отдельно списки идентификаторов и типов полей, чтобы переиспользовать
//!   их в разных повторениях `#( ... )*`;
//! - в `build` для незаданного поля верни ошибку через `ok_or_else`.
//!
//! Заглушка раскрывается в `compile_error!`, пока макрос не реализован.
//! Документация: <https://docs.rs/syn> и <https://docs.rs/quote>.
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive_builder(_input: TokenStream) -> TokenStream {
    quote! { compile_error!("реализуй derive Builder (упражнение не решено)"); }.into()
}
