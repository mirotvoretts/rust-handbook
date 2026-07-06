//! 01 (0x) — Структуры: создание и доступ к полям. Эталонное решение.

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn make_point(x: i32, y: i32) -> Point {
    Point { x, y }
}

pub fn manhattan(p: &Point) -> i32 {
    p.x.abs() + p.y.abs()
}

pub fn translate(p: &Point, dx: i32, dy: i32) -> Point {
    Point {
        x: p.x + dx,
        y: p.y + dy,
    }
}
