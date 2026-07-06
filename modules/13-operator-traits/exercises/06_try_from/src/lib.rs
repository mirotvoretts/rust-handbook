//! 06 (1x) - TryFrom: конверсия с отказом.
//!
//! From обязан быть безотказным; для валидации есть TryFrom с Result. Реализуйте
//! TryFrom<i64> для Age (0..=130) и Percent (0..=100). Тип ошибки - String с
//! информативным текстом (тесты проверяют только Ok/Err, текст - на вашей совести).

#[derive(Debug, PartialEq)]
pub struct Age(pub u8);

#[derive(Debug, PartialEq)]
pub struct Percent(pub u8);

impl TryFrom<i64> for Age {
    type Error = String;

    fn try_from(v: i64) -> Result<Self, Self::Error> {
        todo!("диапазон 0..=130; (0..=130).contains(&v)")
    }
}

impl TryFrom<i64> for Percent {
    type Error = String;

    fn try_from(v: i64) -> Result<Self, Self::Error> {
        todo!("0..=100")
    }
}
