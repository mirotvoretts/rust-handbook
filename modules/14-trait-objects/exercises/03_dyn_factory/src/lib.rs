//! 03 (1x) — Фабрика: тип выбирается в рантайме.
//!
//! Возвращаемый тип функции ОДИН, а конкретная фигура зависит от строки — это ровно
//! тот случай, где без Box<dyn> не обойтись (impl Trait требует один тип на все
//! return'ы). Неизвестный вид -> None.

pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &'static str;
}

pub struct UnitCircle;
pub struct UnitSquare;

impl Shape for UnitCircle {
    fn area(&self) -> f64 {
        std::f64::consts::PI
    }
    fn name(&self) -> &'static str {
        "circle"
    }
}

impl Shape for UnitSquare {
    fn area(&self) -> f64 {
        1.0
    }
    fn name(&self) -> &'static str {
        "square"
    }
}

/// "circle" -> UnitCircle, "square" -> UnitSquare, иначе None.
pub fn make_shape(kind: &str) -> Option<Box<dyn Shape>> {
    todo!("match по kind, Box::new(...)")
}
