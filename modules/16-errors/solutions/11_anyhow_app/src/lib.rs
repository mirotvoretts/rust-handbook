//! 11 (3x) — anyhow: ошибки приложения. Эталонное решение.

use anyhow::{bail, Context, Result};

/// Разбирает строку "число число число ..." и возвращает среднее.
pub fn mean_of_line(line: &str) -> Result<f64> {
    let mut sum = 0.0;
    let mut count = 0usize;
    for token in line.split_whitespace() {
        let x: f64 = token
            .parse()
            .with_context(|| format!("не число: '{token}'"))?;
        sum += x;
        count += 1;
    }
    if count == 0 {
        bail!("пустая строка");
    }
    Ok(sum / count as f64)
}

/// Среднее по нескольким строкам, с контекстом «строка N» при ошибке.
pub fn mean_per_line(lines: &[&str]) -> Result<Vec<f64>> {
    let mut out = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let m = mean_of_line(line).with_context(|| format!("строка {i}"))?;
        out.push(m);
    }
    Ok(out)
}
