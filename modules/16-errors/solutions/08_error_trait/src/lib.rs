//! 08 (2x) — std::error::Error и Box<dyn Error>. Эталонное решение.

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
        write!(f, "строка {} не разбирается", self.line_no)
    }
}

impl Error for BadRecord {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}

/// Парсит каждую строку как i32; первая плохая строка -> BadRecord (line_no с нуля).
pub fn parse_records(lines: &[&str]) -> Result<Vec<i32>, BadRecord> {
    let mut out = Vec::new();
    for (line_no, line) in lines.iter().enumerate() {
        let n = line
            .trim()
            .parse::<i32>()
            .map_err(|cause| BadRecord { line_no, cause })?;
        out.push(n);
    }
    Ok(out)
}

/// Сумма записей; любая ошибка — как Box<dyn Error>.
pub fn sum_records(lines: &[&str]) -> Result<i64, Box<dyn Error>> {
    let records = parse_records(lines)?;
    let mut sum = 0i64;
    for r in records {
        sum += r as i64;
    }
    Ok(sum)
}
