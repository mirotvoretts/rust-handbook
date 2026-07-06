//! 10 (3x) — Orphan rule и newtype. Эталонное решение.

use std::fmt;

/// Newtype над Vec<String>: строка CSV.
pub struct CsvRow(pub Vec<String>);

impl fmt::Display for CsvRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}

impl Default for CsvRow {
    fn default() -> Self {
        Self::new()
    }
}

impl CsvRow {
    /// Пустая строка CSV.
    pub fn new() -> Self {
        CsvRow(Vec::new())
    }

    /// Добавляет ячейку.
    pub fn push(&mut self, cell: &str) {
        self.0.push(cell.to_string());
    }
}
