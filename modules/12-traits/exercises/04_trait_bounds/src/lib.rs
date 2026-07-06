//! 04 (1x) — Границы в обобщённых функциях.
//!
//! Трейт и типы готовы. Напишите две функции: одну с явным параметром `<T: Describe>`,
//! другую — с `impl Describe` в аргументах. Заметьте: у `describe_pair` два аргумента
//! `impl Describe` — это ДВА независимых типа, можно передать Circle и Square.

pub trait Describe {
    fn describe(&self) -> String;
}

pub struct Circle {
    pub radius: f64,
}
pub struct Square {
    pub side: f64,
}

impl Describe for Circle {
    fn describe(&self) -> String {
        format!("круг r={}", self.radius)
    }
}
impl Describe for Square {
    fn describe(&self) -> String {
        format!("квадрат a={}", self.side)
    }
}

/// Описания всех элементов среза (типы элементов одинаковые: T).
pub fn describe_all<T: Describe>(xs: &[T]) -> Vec<String> {
    todo!()
}

/// "{описание a} и {описание b}" — a и b могут быть РАЗНЫХ типов.
pub fn describe_pair(a: &impl Describe, b: &impl Describe) -> String {
    todo!("format! с двумя вызовами describe() через разделитель ' и '")
}
