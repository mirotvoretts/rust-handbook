//! 01 (0x) — Первый трейт.
//!
//! Объявите трейт `Say` с методом `say(&self) -> String` и реализуйте его для `Cat`
//! (возвращает "meow!") и `Dog` (возвращает "woof!"). Реализация — в отдельных
//! impl-блоках, вне определения структур.

pub struct Cat;
pub struct Dog;

pub trait Say {
    // todo!(): сигнатура метода say
    fn say(&self) -> String;
}

// todo!(): impl Say for Cat и impl Say for Dog
impl Say for Cat {
    fn say(&self) -> String {
        todo!()
    }
}

impl Say for Dog {
    fn say(&self) -> String {
        todo!()
    }
}
