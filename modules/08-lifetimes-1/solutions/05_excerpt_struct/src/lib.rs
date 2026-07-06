//! 05 (1x) — Структура с полем-ссылкой (`<'a>` уже задан). Эталонное решение.

pub struct Excerpt<'a> {
    pub part: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn new(part: &'a str) -> Excerpt<'a> {
        Excerpt { part }
    }

    pub fn part(&self) -> &str {
        self.part
    }

    pub fn len(&self) -> usize {
        self.part.len()
    }
}
