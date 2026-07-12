//! 03 (1x) - sealed trait через приватный супертрейт. Эталонное решение.
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
        std::f64::consts::PI * self.radius * self.radius
    }
    fn name(&self) -> &'static str {
        "circle"
    }
}
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    fn name(&self) -> &'static str {
        "square"
    }
}

pub fn describe(shape: &dyn Shape) -> String {
    format!("{} area={:.2}", shape.name(), shape.area())
}
