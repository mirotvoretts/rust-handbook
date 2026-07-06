//! 09 (2x) - Структура, владеющая `String`.
//!
//! `User` владеет своим `name`. Методы чтения берут `&self`, изменения - `&mut self`.
//! Чтобы вернуть заимствованное имя, метод отдаёт `&str` (срез своей строки).

pub struct User {
    name: String,
    age: u32,
}

impl User {
    /// Создаёт пользователя. Имя приходит как `&str`, внутри храните владеющую `String`.
    pub fn new(name: &str, age: u32) -> Self {
        todo!("name.to_string()")
    }

    /// Приветствие вида "Привет, Аня!" (имя - из поля).
    pub fn greet(&self) -> String {
        todo!()
    }

    /// Имя пользователя как строковый срез (без копирования).
    pub fn name(&self) -> &str {
        todo!()
    }

    /// Увеличивает возраст на 1 год.
    pub fn birthday(&mut self) {
        todo!()
    }

    /// Текущий возраст.
    pub fn age(&self) -> u32 {
        todo!()
    }
}
