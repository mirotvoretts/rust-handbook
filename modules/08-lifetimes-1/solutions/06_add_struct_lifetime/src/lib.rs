//! 06 (1x) — ДОБАВЬТЕ `<'a>` САМИ. Эталонное решение.

pub struct Tag<'a> {
    pub name: &'a str,
}

impl<'a> Tag<'a> {
    pub fn new(name: &'a str) -> Tag<'a> {
        Tag { name }
    }

    pub fn name(&self) -> &str {
        self.name
    }
}
