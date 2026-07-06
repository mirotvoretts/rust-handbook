//! 02 (0x) — Default-метод. Эталонное решение.

pub trait Greet {
    fn name(&self) -> String;

    /// Default: "Привет, {name}!"
    fn greet(&self) -> String {
        format!("Привет, {}!", self.name())
    }
}

pub struct Robot;
pub struct Pirate;

impl Greet for Robot {
    fn name(&self) -> String {
        String::from("R2")
    }
}

impl Greet for Pirate {
    fn name(&self) -> String {
        String::from("Флинт")
    }

    fn greet(&self) -> String {
        format!("Йо-хо-хо, {}!", self.name())
    }
}
