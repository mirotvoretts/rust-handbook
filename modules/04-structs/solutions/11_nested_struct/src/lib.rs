//! 11 (2x) — Структуры из структур (композиция). Эталонное решение.

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }

    pub fn length(&self) -> f64 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn midpoint(&self) -> Point {
        Point {
            x: (self.start.x + self.end.x) / 2.0,
            y: (self.start.y + self.end.y) / 2.0,
        }
    }
}
