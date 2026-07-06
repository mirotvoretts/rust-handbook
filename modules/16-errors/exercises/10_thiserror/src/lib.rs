//! 10 (3x) - thiserror: тот же канон без бойлерплейта.
//!
//! Перепишите руками написанное в 07/08 на derive: #[derive(thiserror::Error)] +
//! атрибуты #[error("...")] на вариантах и #[from] на обёртках. Display, Error и
//! From генерируются сами. Допишите варианты и функцию.

use std::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// todo!(): атрибут #[error] с текстом про отсутствующий ключ {0}
    #[error("нет ключа: {0}")]
    MissingKey(String),

    // todo!(): вариант BadNumber(#[from] ParseIntError) с атрибутом #[error]
    #[error("не число")]
    BadNumber(#[from] ParseIntError),
}

/// Ищет "key=value"; ошибки - через новый тип.
pub fn get_number(lines: &[&str], key: &str) -> Result<i64, ConfigError> {
    todo!("найдите строку с префиксом key= (MissingKey, если нет), parse::<i64>()?")
}
