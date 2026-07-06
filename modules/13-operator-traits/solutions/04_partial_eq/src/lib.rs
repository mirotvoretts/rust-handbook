//! 04 (1x) — Ручной PartialEq и сравнение разных типов. Эталонное решение.

#[derive(Debug)]
pub struct Username(pub String);

impl PartialEq for Username {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl PartialEq<str> for Username {
    fn eq(&self, other: &str) -> bool {
        self.0.eq_ignore_ascii_case(other)
    }
}
