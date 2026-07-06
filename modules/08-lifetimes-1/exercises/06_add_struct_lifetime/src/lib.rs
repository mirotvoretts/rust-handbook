//! 06 (1x) — ДОБАВЬТЕ `<'a>` САМИ.
//!
//! Крейт НЕ КОМПИЛИРУЕТСЯ: `error[E0106]: missing lifetime specifier` на поле-ссылке.
//! Структура хранит ссылку, значит обязана объявить параметр времени жизни. Добавьте `<'a>`:
//!   - в объявление структуры: `struct Tag<'a>`;
//!   - к полю: `name: &'a str`;
//!   - в блок impl: `impl<'a> Tag<'a>` и в сигнатуры, где это нужно.

pub struct Tag {
    pub name: &str,
}

impl Tag {
    pub fn new(name: &str) -> Tag {
        Tag { name }
    }

    pub fn name(&self) -> &str {
        self.name
    }
}
