//! 01 (0x) - Enum без данных и методы через `match`.
//!
//! `Direction` - перечисление из четырёх вариантов. Реализуйте функции, разбирающие его
//! через исчерпывающий `match`.

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

/// Противоположное направление.
pub fn opposite(d: Direction) -> Direction {
    todo!("match по всем четырём вариантам")
}

/// Следующее направление по часовой стрелке: N -> E -> S -> W -> N.
pub fn clockwise(d: Direction) -> Direction {
    todo!()
}

/// Азимут в градусах: N=0, E=90, S=180, W=270.
pub fn to_degrees(d: Direction) -> u32 {
    todo!()
}
