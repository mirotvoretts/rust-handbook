//! 03 (0x) - From и бесплатный Into. Эталонное решение.

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Meters(pub f64);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Centimeters(pub f64);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Feet(pub f64);

impl From<Meters> for Centimeters {
    fn from(m: Meters) -> Self {
        Centimeters(m.0 * 100.0)
    }
}

impl From<Centimeters> for Meters {
    fn from(cm: Centimeters) -> Self {
        Meters(cm.0 / 100.0)
    }
}

impl From<Feet> for Meters {
    fn from(ft: Feet) -> Self {
        Meters(ft.0 * 0.3048)
    }
}
