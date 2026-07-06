//! 02 (0x) - Deref coercion: Box<T> ведёт себя как &T.
//!
//! Методы T доступны прямо на Box<T> (метод-резолвинг авто-разыменовывает),
//! а &Box<String> при передаче в функцию приводится к &str.
// Box<String> берётся по значению намеренно (демонстрация коерции) - глушим clippy.
#![allow(clippy::boxed_local)]

/// Длина строки внутри Box. Вызови .len() ПРЯМО на Box<String>
/// (коерция добирается до метода str).
pub fn boxed_len(s: Box<String>) -> usize {
    todo!("s.len()")
}

/// Дана функция, принимающая &str.
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

/// Поприветствуй имя из Box<String> через greet: передай &name,
/// и &Box<String> -> &String -> &str сработает автоматически.
pub fn greet_boxed(name: Box<String>) -> String {
    todo!("greet(&name)")
}

/// Первый символ строки в Box (или None для пустой), через методы str.
pub fn first_char(s: Box<String>) -> Option<char> {
    todo!("s.chars().next()")
}
