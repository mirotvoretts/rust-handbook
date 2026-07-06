//! 03 (1x) — Фабрика: тип выбирается в рантайме. Эталонное решение.

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
    match kind {
        "circle" => Some(Box::new(UnitCircle)),
        "square" => Some(Box::new(UnitSquare)),
        _ => None,
    }
}
