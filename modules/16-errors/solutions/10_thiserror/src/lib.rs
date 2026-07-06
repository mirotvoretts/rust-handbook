//! 10 (3x) - thiserror: тот же канон без бойлерплейта. Эталонное решение.

use std::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("нет ключа: {0}")]
    MissingKey(String),

    #[error("не число")]
    BadNumber(#[from] ParseIntError),
}

/// Ищет "key=value"; ошибки - через новый тип.
pub fn get_number(lines: &[&str], key: &str) -> Result<i64, ConfigError> {
    for line in lines {
        if let Some(v) = line.strip_prefix(key).and_then(|r| r.strip_prefix('=')) {
            return Ok(v.trim().parse::<i64>()?); // From из #[from]
        }
    }
    Err(ConfigError::MissingKey(key.to_string()))
}
