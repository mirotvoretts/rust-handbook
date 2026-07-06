//! 03 (1x) - Параметр трейта: много реализаций на один тип. Эталонное решение.

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
        Miles(self.0 * 0.621371)
    }
}

impl Convert<MetersOut> for Kilometers {
    fn convert(&self) -> MetersOut {
        MetersOut(self.0 * 1000.0)
    }
}
