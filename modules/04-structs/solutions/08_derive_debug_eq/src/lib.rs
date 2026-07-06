//! 08 (1x) - Вывод типажей через `#[derive(...)]`. Эталонное решение.

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn same(a: &Point, b: &Point) -> bool {
    a == b
}

pub fn debug_string(p: &Point) -> String {
    format!("{p:?}")
}
