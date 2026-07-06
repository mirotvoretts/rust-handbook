//! 01 (0x) - Ассоциированный тип.
//!
//! У каждого парсера ровно один тип результата - значит, это ассоциированный тип, а
//! не параметр. Реализуйте оба парсера; заметьте, что вызовы parse не требуют
//! turbofish - Output выводится из типа парсера.

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
        todo!("str::parse + .ok()")
    }
}

impl Parser for CsvParser {
    type Output = Vec<String>;

    fn parse(&self, input: &str) -> Option<Vec<String>> {
        todo!("split(',') + trim; пустая строка -> Some(vec![])... нет: пустая строка -> None")
    }
}
