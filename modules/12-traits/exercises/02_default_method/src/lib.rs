//! 02 (0x) - Default-метод.
//!
//! Трейт `Greet`: обязательный `name()` и default-метод `greet()`, который строит
//! приветствие "Привет, {name}!". `Robot` довольствуется default'ом, `Pirate`
//! переопределяет greet на "Йо-хо-хо, {name}!".

pub trait Greet {
    fn name(&self) -> String;

    /// Default: "Привет, {name}!"
    fn greet(&self) -> String {
        todo!("выразите через self.name()")
    }
}

pub struct Robot;
pub struct Pirate;

impl Greet for Robot {
    fn name(&self) -> String {
        String::from("R2")
    }
    // greet - из default'а
}

impl Greet for Pirate {
    fn name(&self) -> String {
        String::from("Флинт")
    }

    fn greet(&self) -> String {
        todo!("переопределите на пиратское приветствие с self.name()")
    }
}
