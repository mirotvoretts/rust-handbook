//! 06 (1x) — Tuple-структуры.
//!
//! У tuple-структуры поля без имён — доступ по индексам `.0`, `.1`, `.2`. Методы работают так же.

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    /// Суммарная яркость: сумма трёх компонент (как `u32`, чтобы не переполнить).
    pub fn brightness(&self) -> u32 {
        todo!("self.0 + self.1 + self.2 как u32")
    }

    /// Инвертированный цвет: каждая компонента заменяется на 255 - компонента.
    pub fn invert(&self) -> Rgb {
        todo!()
    }
}
