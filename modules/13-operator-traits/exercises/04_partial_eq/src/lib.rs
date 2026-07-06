//! 04 (1x) - Ручной PartialEq и сравнение разных типов.
//!
//! Username сравнивается без учёта ASCII-регистра: "Alice" == "alice". Два impl:
//! PartialEq (Username == Username) и PartialEq<str> (Username == *"alice").
//! Подсказка: у str есть метод eq_ignore_ascii_case.

#[derive(Debug)]
pub struct Username(pub String);

impl PartialEq for Username {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialEq<str> for Username {
    fn eq(&self, other: &str) -> bool {
        todo!()
    }
}
