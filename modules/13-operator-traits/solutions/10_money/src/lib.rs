//! 10 (3x) — Капстоун: деньги. Эталонное решение.

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
        if self.currency != rhs.currency {
            panic!(
                "нельзя сложить {} и {}",
                self.currency.code(),
                rhs.currency.code()
            );
        }
        Money { kopecks: self.kopecks + rhs.kopecks, currency: self.currency }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{:02} {}",
            self.kopecks / 100,
            self.kopecks % 100,
            self.currency.code()
        )
    }
}

impl TryFrom<&str> for Money {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut parts = s.split_whitespace();
        let amount = parts.next().ok_or("пустая строка")?;
        let code = parts.next().ok_or("нет кода валюты")?;
        if parts.next().is_some() {
            return Err(format!("лишние данные в '{s}'"));
        }

        let currency = match code {
            "RUB" => Currency::Rub,
            "USD" => Currency::Usd,
            other => return Err(format!("неизвестная валюта '{other}'")),
        };

        let kopecks = match amount.split_once('.') {
            None => {
                let whole: i64 =
                    amount.parse().map_err(|_| format!("не число: '{amount}'"))?;
                whole * 100
            }
            Some((whole, frac)) => {
                if frac.len() != 2 {
                    return Err(format!("копейки должны быть двумя цифрами: '{frac}'"));
                }
                let whole: i64 =
                    whole.parse().map_err(|_| format!("не число: '{whole}'"))?;
                let frac: i64 =
                    frac.parse().map_err(|_| format!("не число: '{frac}'"))?;
                whole * 100 + frac
            }
        };

        Ok(Money { kopecks, currency })
    }
}
