//! 02 (0x) — &dyn без аллокаций.
//!
//! Trait object не обязан жить в Box: &dyn Trait — заём. Реализуйте Loud для типов и
//! shout_all, принимающую срез ссылок. Заметьте сигнатуру &[&dyn Loud] — вектор
//! ссылок на разнотипные значения, живущие на стеке вызывающего.

pub trait Loud {
    fn shout(&self) -> String;
}

pub struct Siren;
pub struct Speaker {
    pub volume: u32,
}

impl Loud for Siren {
    fn shout(&self) -> String {
        todo!("WEE-OO")
    }
}

impl Loud for Speaker {
    fn shout(&self) -> String {
        todo!("BOOM x volume: строка 'BOOM' повторённая volume раз — метод repeat")
    }
}

/// Склеивает крики всех через пробел.
pub fn shout_all(xs: &[&dyn Loud]) -> String {
    todo!("соберите Vec<String> и join(\" \")")
}
