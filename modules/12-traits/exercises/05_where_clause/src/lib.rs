//! 05 (1x) — where-клаузы и составные границы.
//!
//! Товары: у типа может быть цена (Priced) и имя (Named). Функции требуют ОБЕ
//! способности — запишите границы через `where`. Подсказка к cheapest_name: обходите
//! срез, храня ссылку на лучшего кандидата (как largest в M11).

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

/// Имя самого дешёвого товара (None для пустого среза). При равенстве цен — первый.
pub fn cheapest_name<T>(xs: &[T]) -> Option<String>
where
    T: Priced + Named,
{
    todo!()
}

/// Суммарная цена + список имён: (total, names). Два НЕЗАВИСИМЫХ параметра типа.
pub fn receipt<A, B>(a: &A, b: &B) -> (u32, Vec<String>)
where
    A: Priced + Named,
    B: Priced + Named,
{
    todo!()
}
