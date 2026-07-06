//! 01 (0x) - Структуры: создание и доступ к полям.
//!
//! Тип `Point` уже объявлен. Реализуйте функции, создающие `Point` и читающие его поля.

pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Создаёт точку с заданными координатами.
pub fn make_point(x: i32, y: i32) -> Point {
    todo!("создайте Point с полями x и y")
}

/// Манхэттенское расстояние от начала координат: |x| + |y|.
pub fn manhattan(p: &Point) -> i32 {
    todo!("p.x.abs() + p.y.abs()")
}

/// Возвращает новую точку, сдвинутую на (dx, dy).
pub fn translate(p: &Point, dx: i32, dy: i32) -> Point {
    todo!()
}
