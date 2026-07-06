//! 11 (2x) — Тип `!` (never) в позиции значения. Эталонное решение.

pub fn parse_bit(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        _ => unreachable!(),
    }
}

pub fn get_or_panic(opt: Option<i32>) -> i32 {
    match opt {
        Some(v) => v,
        None => panic!("empty"),
    }
}

pub fn checked_div(n: i32, d: i32) -> i32 {
    if d == 0 {
        panic!("div by zero")
    } else {
        n / d
    }
}
