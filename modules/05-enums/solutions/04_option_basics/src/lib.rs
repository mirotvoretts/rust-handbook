//! 04 (0x) — Создание и разбор `Option`. Эталонное решение.

pub fn safe_sqrt(x: f64) -> Option<f64> {
    if x < 0.0 {
        None
    } else {
        Some(x.sqrt())
    }
}

pub fn first(xs: &[i32]) -> Option<i32> {
    if xs.is_empty() {
        None
    } else {
        Some(xs[0])
    }
}

pub fn describe(opt: Option<i32>) -> String {
    match opt {
        Some(n) => format!("got {n}"),
        None => String::from("nothing"),
    }
}
