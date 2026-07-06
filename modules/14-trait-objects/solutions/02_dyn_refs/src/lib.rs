//! 02 (0x) - &dyn без аллокаций. Эталонное решение.

pub trait Loud {
    fn shout(&self) -> String;
}

pub struct Siren;
pub struct Speaker {
    pub volume: u32,
}

impl Loud for Siren {
    fn shout(&self) -> String {
        String::from("WEE-OO")
    }
}

impl Loud for Speaker {
    fn shout(&self) -> String {
        "BOOM".repeat(self.volume as usize)
    }
}

/// Склеивает крики всех через пробел.
pub fn shout_all(xs: &[&dyn Loud]) -> String {
    let mut parts = Vec::new();
    for x in xs {
        parts.push(x.shout());
    }
    parts.join(" ")
}
