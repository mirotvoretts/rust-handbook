//! 01 (0x) — Первый трейт. Эталонное решение.

pub struct Cat;
pub struct Dog;

pub trait Say {
    fn say(&self) -> String;
}

impl Say for Cat {
    fn say(&self) -> String {
        String::from("meow!")
    }
}

impl Say for Dog {
    fn say(&self) -> String {
        String::from("woof!")
    }
}
