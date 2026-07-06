//! 05 (1x) - where-клаузы и составные границы. Эталонное решение.

pub trait Priced {
    /// Цена в копейках.
    fn price(&self) -> u32;
}

pub trait Named {
    fn name(&self) -> String;
}

pub struct Book {
    pub title: String,
    pub kopecks: u32,
}

impl Priced for Book {
    fn price(&self) -> u32 {
        self.kopecks
    }
}
impl Named for Book {
    fn name(&self) -> String {
        self.title.clone()
    }
}

/// Имя самого дешёвого товара (None для пустого среза). При равенстве цен - первый.
pub fn cheapest_name<T>(xs: &[T]) -> Option<String>
where
    T: Priced + Named,
{
    let mut best = xs.first()?;
    for x in &xs[1..] {
        if x.price() < best.price() {
            best = x;
        }
    }
    Some(best.name())
}

/// Суммарная цена + список имён: (total, names). Два НЕЗАВИСИМЫХ параметра типа.
pub fn receipt<A, B>(a: &A, b: &B) -> (u32, Vec<String>)
where
    A: Priced + Named,
    B: Priced + Named,
{
    (a.price() + b.price(), vec![a.name(), b.name()])
}
