//! 07 (2x) - From-конверсии и ?.
//!
//! У load_port два источника ошибок с разными типами. Реализуйте обе From-конверсии -
//! и оба ? заработают без map_err. Именно эта связка делает свои типы ошибок
//! практичными.

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    MissingKey(String),
    BadNumber(String),
}

/// "Ошибка отсутствия ключа" приходит как String из lookup.
impl From<String> for ConfigError {
    fn from(key: String) -> Self {
        todo!("MissingKey")
    }
}

impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        todo!("BadNumber с текстом ошибки (e.to_string())")
    }
}

/// Ищет "key=value" в строках; Err(String с именем ключа), если нет.
fn lookup<'a>(lines: &'a [&str], key: &str) -> Result<&'a str, String> {
    for line in lines {
        if let Some(v) = line.strip_prefix(key).and_then(|r| r.strip_prefix('=')) {
            return Ok(v);
        }
    }
    Err(key.to_string())
}

/// Достаёт порт из конфига: lookup? + parse? - обе ошибки конвертируются сами.
pub fn load_port(lines: &[&str]) -> Result<u16, ConfigError> {
    todo!("две строки с двумя ?")
}
