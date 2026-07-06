//! 05 (1x) — Свой enum-ошибка + Display. Эталонное решение.

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
        match self {
            NicknameError::TooShort { len } => write!(f, "слишком короткий: {len} < 3"),
            NicknameError::TooLong { len } => write!(f, "слишком длинный: {len} > 12"),
            NicknameError::BadChar { ch } => write!(f, "недопустимый символ '{ch}'"),
            NicknameError::BadStart { ch } => write!(f, "должен начинаться с буквы, а не '{ch}'"),
        }
    }
}

/// Проверяет ник, возвращает его же (владение) при успехе.
pub fn validate_nickname(name: String) -> Result<String, NicknameError> {
    let len = name.chars().count();
    if len < 3 {
        return Err(NicknameError::TooShort { len });
    }
    if len > 12 {
        return Err(NicknameError::TooLong { len });
    }
    let first = name.chars().next().expect("len >= 3");
    if !first.is_ascii_alphabetic() {
        return Err(NicknameError::BadStart { ch: first });
    }
    for ch in name.chars() {
        if !(ch.is_ascii_alphanumeric() || ch == '_') {
            return Err(NicknameError::BadChar { ch });
        }
    }
    Ok(name)
}
