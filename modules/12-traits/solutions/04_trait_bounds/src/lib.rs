//! 04 (1x) - Границы в обобщённых функциях. Эталонное решение.

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
    let mut out = Vec::new();
    for x in xs {
        out.push(x.describe());
    }
    out
}

/// "{описание a} и {описание b}" - a и b могут быть РАЗНЫХ типов.
pub fn describe_pair(a: &impl Describe, b: &impl Describe) -> String {
    format!("{} и {}", a.describe(), b.describe())
}
