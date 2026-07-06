//! 07 (2x) - From-конверсии и ?. Эталонное решение.

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    MissingKey(String),
    BadNumber(String),
}

impl From<String> for ConfigError {
    fn from(key: String) -> Self {
        ConfigError::MissingKey(key)
    }
}

impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::BadNumber(e.to_string())
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

/// Достаёт порт из конфига.
pub fn load_port(lines: &[&str]) -> Result<u16, ConfigError> {
    let raw = lookup(lines, "port")?;
    let port = raw.trim().parse::<u16>()?;
    Ok(port)
}
