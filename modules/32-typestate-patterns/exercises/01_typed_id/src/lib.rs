//! 01 (0x) - типизированный id на PhantomData.
//!
//! Реализуй типизированный идентификатор `Id<T>` поверх `PhantomData<T>`, чтобы
//! значение типа `Id<User>` нельзя было передать туда, где ожидается `Id<Post>`.
//! Параметр `T` служит только меткой типа и не хранит данных.
//!
//! Тебе нужно заполнить тела:
//! - `Id::new(value)` и `Id::value(&self)`;
//! - реализаций `Clone`, `Copy`, `PartialEq`, `Debug` (формат `"Id(<value>)"`);
//! - функции `user_name(id: Id<User>) -> String` (формат `"user #<value>"`).
//!
//! Подсказки:
//! - поле `_marker: PhantomData<T>` инициализируется значением `PhantomData`;
//! - `Copy` реализуется пустым `impl`, но требует наличия `Clone`.
//!
//! Заглушки помечены `todo!()` - крейт компилируется, но тесты падают, пока не решено.
//! Документация PhantomData:
//! <https://doc.rust-lang.org/std/marker/struct.PhantomData.html>.
use std::marker::PhantomData;

pub struct User;
pub struct Post;

pub struct Id<T> {
    value: u64,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(_value: u64) -> Self {
        todo!()
    }
    pub fn value(&self) -> u64 {
        todo!()
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}
impl<T> Copy for Id<T> {}
impl<T> PartialEq for Id<T> {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
impl<T> std::fmt::Debug for Id<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub fn user_name(_id: Id<User>) -> String {
    todo!()
}
