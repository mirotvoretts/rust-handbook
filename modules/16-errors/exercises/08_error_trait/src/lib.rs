//! 08 (2x) — std::error::Error и Box<dyn Error>.
//!
//! Реализуйте Error с source() (цепочка причин) и функцию, возвращающую
//! Box<dyn Error> — «любая ошибка»: обе наших ошибки в неё конвертируются
//! автоматически (в std есть blanket From<E: Error> для Box<dyn Error>).

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct BadRecord {
    pub line_no: usize,
    pub cause: ParseIntError,
}

impl fmt::Display for BadRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("например: строка N не разбирается")
    }
}

impl Error for BadRecord {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!("Some(&self.cause)")
    }
}

/// Парсит каждую строку как i32; первая плохая строка -> BadRecord (line_no с нуля).
pub fn parse_records(lines: &[&str]) -> Result<Vec<i32>, BadRecord> {
    todo!("map_err с конструированием BadRecord")
}

/// Сумма записей; любая ошибка — как Box<dyn Error> (просто ?).
pub fn sum_records(lines: &[&str]) -> Result<i64, Box<dyn Error>> {
    todo!("parse_records(lines)? — конверсия в Box сама")
}
