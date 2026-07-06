//! 10 (3x) — Orphan rule и newtype.
//!
//! `impl Display for Vec<String>` запрещён: и трейт, и тип — чужие (E0117). Обёртка
//! `CsvRow(Vec<String>)` делает тип вашим — и impl становится законным. Реализуйте
//! Display (значения через запятую-пробел) и метод push. Заготовка fmt: сигнатуру
//! менять нельзя, пишите в f через write!(f, ...).

use std::fmt;

/// Newtype над Vec<String>: строка CSV.
pub struct CsvRow(pub Vec<String>);

impl fmt::Display for CsvRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("склейте self.0.join и запишите результат в f макросом write!")
    }
}

impl CsvRow {
    /// Пустая строка CSV.
    pub fn new() -> Self {
        todo!()
    }

    /// Добавляет ячейку.
    pub fn push(&mut self, cell: &str) {
        todo!("доступ к внутреннему вектору — self.0")
    }
}
