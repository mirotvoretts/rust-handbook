//! 11 (2x) - Метод, возвращающий более длинное из двух полей-ссылок. Эталонное решение.

pub struct Pair<'a> {
    pub a: &'a str,
    pub b: &'a str,
}

impl<'a> Pair<'a> {
    pub fn new(a: &'a str, b: &'a str) -> Pair<'a> {
        Pair { a, b }
    }

    pub fn longer(&self) -> &'a str {
        if self.a.len() >= self.b.len() {
            self.a
        } else {
            self.b
        }
    }
}
