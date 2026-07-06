//! 05 (1x) — Свой enum-ошибка + Display.
//!
//! Валидация ника: 3..=12 символов, только ASCII-буквы/цифры/подчёркивание, первый
//! символ — буква. Каждый отказ — свой вариант ошибки с данными. Display пишет
//! человекочитаемый текст (точный текст тесты не проверяют — только вариант).

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum NicknameError {
    TooShort { len: usize },
    TooLong { len: usize },
    BadChar { ch: char },
    BadStart { ch: char },
}

impl fmt::Display for NicknameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("match по вариантам, write! с деталями")
    }
}

/// Проверяет ник, возвращает его же (владение) при успехе.
pub fn validate_nickname(name: String) -> Result<String, NicknameError> {
    todo!("chars().count() для длины; is_ascii_alphanumeric; порядок проверок: длина, старт, символы")
}
