//! 01 (0x) - Ассоциированный тип. Эталонное решение.

pub trait Parser {
    type Output;

    /// None - если вход не разбирается.
    fn parse(&self, input: &str) -> Option<Self::Output>;
}

/// Парсит целое число.
pub struct IntParser;

/// Разбивает строку по запятым (и обрезает пробелы вокруг кусков).
pub struct CsvParser;

impl Parser for IntParser {
    type Output = i32;

    fn parse(&self, input: &str) -> Option<i32> {
        input.trim().parse().ok()
    }
}

impl Parser for CsvParser {
    type Output = Vec<String>;

    fn parse(&self, input: &str) -> Option<Vec<String>> {
        if input.is_empty() {
            return None;
        }
        let mut out = Vec::new();
        for part in input.split(',') {
            out.push(part.trim().to_string());
        }
        Some(out)
    }
}
