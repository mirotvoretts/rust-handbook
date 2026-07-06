//! 02 (0x) - Deref coercion. Эталонное решение.
// Box<String> берётся по значению намеренно (демонстрация коерции) - глушим clippy.
#![allow(clippy::boxed_local)]

/// .len() через Box<String> (коерция до метода str).
pub fn boxed_len(s: Box<String>) -> usize {
    s.len()
}

/// Функция, принимающая &str.
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

/// greet(&name): &Box<String> -> &str автоматически.
pub fn greet_boxed(name: Box<String>) -> String {
    greet(&name)
}

/// Первый символ через методы str.
pub fn first_char(s: Box<String>) -> Option<char> {
    s.chars().next()
}
