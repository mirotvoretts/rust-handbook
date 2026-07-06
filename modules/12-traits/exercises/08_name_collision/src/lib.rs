//! 08 (2x) - Коллизии имён и fully qualified syntax.
//!
//! У `Human` ТРИ метода fly: inherent + два трейтовых. Реализуйте все три так, чтобы
//! тесты прошли, и допишите функции-диспетчеры: обычный вызов `h.fly()` берёт
//! inherent-метод, для трейтовых нужен `Pilot::fly(&h)` / `<Human as Wizard>::fly(&h)`.

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
        todo!()
    }
}

impl Pilot for Human {
    /// "выполняю рейс".
    fn fly(&self) -> String {
        todo!()
    }
}

impl Wizard for Human {
    /// "лечу на метле".
    fn fly(&self) -> String {
        todo!()
    }
}

/// Вызывает ИМЕННО трейтовый метод Pilot.
pub fn fly_as_pilot(h: &Human) -> String {
    todo!("Pilot::fly(h) или <Human as Pilot>::fly(h)")
}

/// Вызывает ИМЕННО трейтовый метод Wizard.
pub fn fly_as_wizard(h: &Human) -> String {
    todo!()
}
