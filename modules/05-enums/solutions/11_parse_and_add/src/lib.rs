//! 11 (2x) — Проброска ошибки вручную через `match`. Эталонное решение.

pub fn parse_and_add(a: &str, b: &str) -> Result<i32, String> {
    let x = match a.parse::<i32>() {
        Ok(v) => v,
        Err(_) => return Err(format!("bad number: {a}")),
    };
    let y = match b.parse::<i32>() {
        Ok(v) => v,
        Err(_) => return Err(format!("bad number: {b}")),
    };
    Ok(x + y)
}
