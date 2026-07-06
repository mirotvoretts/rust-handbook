//! 10 (3x) — Капстоун: деньги.
//!
//! Money хранит сумму в КОПЕЙКАХ (i64) и код валюты. Реализуйте:
//! - Add: суммы складываются; РАЗНЫЕ валюты — panic! (в M16 узнаем способ лучше);
//! - Display: "12.50 RUB" (две цифры после точки, кратно копейкам);
//! - TryFrom<&str>: парсинг "12.50 RUB" / "7 USD" (без знака минус; ошибки — String);
//! - Ord внутри одной валюты нам не нужен — сравнение уже есть через derive(PartialEq).
//!
//! Подсказка к TryFrom: split_whitespace, затем split('.') для суммы.

use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Currency {
    Rub,
    Usd,
}

impl Currency {
    pub fn code(&self) -> &'static str {
        match self {
            Currency::Rub => "RUB",
            Currency::Usd => "USD",
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Money {
    pub kopecks: i64,
    pub currency: Currency,
}

impl std::ops::Add for Money {
    type Output = Money;

    fn add(self, rhs: Money) -> Money {
        todo!("разные валюты -> panic! с внятным сообщением")
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("рубли = kopecks / 100, копейки = kopecks % 100, ширина копеек — 2 знака")
    }
}

impl TryFrom<&str> for Money {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        todo!("формат: '<целое>[.<копейки-2-цифры>] <RUB|USD>'")
    }
}
