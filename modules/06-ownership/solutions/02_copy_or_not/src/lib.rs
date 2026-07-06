//! 02 (0x) — `Copy` против не-`Copy`. Эталонное решение.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Возвращает две копии точки. Компилируется только если `Point: Copy`.
pub fn duplicate(p: Point) -> (Point, Point) {
    (p, p)
}

// `Copy` невозможен: тип владеет String (кучей), побитовая копия дала бы double-free.
#[derive(Clone, Debug, PartialEq)]
pub struct Label(pub String);

/// Возвращает две независимые копии метки через явный clone.
pub fn clone_pair(l: &Label) -> (Label, Label) {
    (l.clone(), l.clone())
}
