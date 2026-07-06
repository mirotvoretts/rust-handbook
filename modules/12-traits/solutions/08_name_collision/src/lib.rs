//! 08 (2x) - Коллизии имён и fully qualified syntax. Эталонное решение.

pub trait Pilot {
    fn fly(&self) -> String;
}

pub trait Wizard {
    fn fly(&self) -> String;
}

pub struct Human;

impl Human {
    /// Inherent: "машу руками".
    pub fn fly(&self) -> String {
        String::from("машу руками")
    }
}

impl Pilot for Human {
    /// "выполняю рейс".
    fn fly(&self) -> String {
        String::from("выполняю рейс")
    }
}

impl Wizard for Human {
    /// "лечу на метле".
    fn fly(&self) -> String {
        String::from("лечу на метле")
    }
}

/// Вызывает ИМЕННО трейтовый метод Pilot.
pub fn fly_as_pilot(h: &Human) -> String {
    Pilot::fly(h)
}

/// Вызывает ИМЕННО трейтовый метод Wizard.
pub fn fly_as_wizard(h: &Human) -> String {
    <Human as Wizard>::fly(h)
}
