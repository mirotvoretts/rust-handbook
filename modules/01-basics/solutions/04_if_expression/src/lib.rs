//! 04 (0x) - `if` как выражение. Эталонное решение.

pub fn sign(n: i32) -> i32 {
    if n < 0 {
        -1
    } else if n > 0 {
        1
    } else {
        0
    }
}

pub fn grade(score: u32) -> char {
    if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else if score >= 60 {
        'D'
    } else {
        'F'
    }
}

pub fn abs_diff(a: i32, b: i32) -> i32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}
