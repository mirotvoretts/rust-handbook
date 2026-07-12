//! 03 (1x) - sealed trait через приватный супертрейт.
//!
//! Трейт `Shape` (методы `area` и `name`) должен быть запечатан (sealed): его
//! нельзя реализовать за пределами этого крейта. Приём - приватный супертрейт
//! `sealed::Sealed`, реализовать который снаружи невозможно. Скелет запечатывания
//! уже дан; тебе остаётся заполнить геометрию.
//!
//! Заполни тела:
//! - `Circle::area` (площадь круга, `PI * r * r`) и `Circle::name` (`"circle"`);
//! - `Square::area` (`side * side`) и `Square::name` (`"square"`);
//! - `describe(&dyn Shape)` в формате `"<name> area=<area>"` с двумя знаками после
//!   запятой (например `"square area=9.00"`).
//!
//! Заглушки помечены `todo!()` - крейт компилируется, тесты падают, пока не решено.
//! Про sealed-трейты:
//! <https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed>.
mod sealed {
    pub trait Sealed {}
}

pub trait Shape: sealed::Sealed {
    fn area(&self) -> f64;
    fn name(&self) -> &'static str;
}

pub struct Circle {
    pub radius: f64,
}
pub struct Square {
    pub side: f64,
}

impl sealed::Sealed for Circle {}
impl sealed::Sealed for Square {}

impl Shape for Circle {
    fn area(&self) -> f64 {
        todo!()
    }
    fn name(&self) -> &'static str {
        todo!()
    }
}
impl Shape for Square {
    fn area(&self) -> f64 {
        todo!()
    }
    fn name(&self) -> &'static str {
        todo!()
    }
}

pub fn describe(_shape: &dyn Shape) -> String {
    todo!()
}
