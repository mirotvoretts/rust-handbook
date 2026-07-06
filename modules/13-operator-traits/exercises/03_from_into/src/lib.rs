//! 03 (0x) — From и бесплатный Into.
//!
//! Реализуйте From-конверсии между единицами. Into НЕ реализуйте — он появится сам
//! (blanket impl в std: `impl<T, U: From<T>> Into<U> for T`). Тесты вызывают .into().

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Meters(pub f64);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Centimeters(pub f64);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Feet(pub f64);

impl From<Meters> for Centimeters {
    fn from(m: Meters) -> Self {
        todo!("x100")
    }
}

impl From<Centimeters> for Meters {
    fn from(cm: Centimeters) -> Self {
        todo!()
    }
}

impl From<Feet> for Meters {
    fn from(ft: Feet) -> Self {
        todo!("1 фут = 0.3048 м")
    }
}
