//! 06 (1x) — TryFrom: конверсия с отказом. Эталонное решение.

#[derive(Debug, PartialEq)]
pub struct Age(pub u8);

#[derive(Debug, PartialEq)]
pub struct Percent(pub u8);

impl TryFrom<i64> for Age {
    type Error = String;

    fn try_from(v: i64) -> Result<Self, Self::Error> {
        if (0..=130).contains(&v) {
            Ok(Age(v as u8))
        } else {
            Err(format!("возраст вне диапазона: {v}"))
        }
    }
}

impl TryFrom<i64> for Percent {
    type Error = String;

    fn try_from(v: i64) -> Result<Self, Self::Error> {
        if (0..=100).contains(&v) {
            Ok(Percent(v as u8))
        } else {
            Err(format!("процент вне диапазона: {v}"))
        }
    }
}
