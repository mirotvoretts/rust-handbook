//! 06 (1x) — Tuple-структуры. Эталонное решение.

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub fn brightness(&self) -> u32 {
        self.0 as u32 + self.1 as u32 + self.2 as u32
    }

    pub fn invert(&self) -> Rgb {
        Rgb(255 - self.0, 255 - self.1, 255 - self.2)
    }
}
