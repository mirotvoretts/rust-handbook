//! 03 (1x) - Параметр трейта: много реализаций на один тип.
//!
//! Convert<T> параметризован - поэтому Kilometers конвертируется И в мили, И в метры
//! (два impl на один тип; с ассоциированным типом это было бы невозможно). Заметьте:
//! вызовы требуют уточнения типа - цена множественности.

pub struct Kilometers(pub f64);

#[derive(Debug, PartialEq)]
pub struct Miles(pub f64);

#[derive(Debug, PartialEq)]
pub struct MetersOut(pub f64);

pub trait Convert<T> {
    fn convert(&self) -> T;
}

impl Convert<Miles> for Kilometers {
    fn convert(&self) -> Miles {
        todo!("1 км = 0.621371 мили")
    }
}

impl Convert<MetersOut> for Kilometers {
    fn convert(&self) -> MetersOut {
        todo!("x1000")
    }
}
