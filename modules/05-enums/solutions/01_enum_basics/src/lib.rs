//! 01 (0x) - Enum без данных и методы через `match`. Эталонное решение.

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub fn opposite(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::South,
        Direction::East => Direction::West,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
    }
}

pub fn clockwise(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

pub fn to_degrees(d: Direction) -> u32 {
    match d {
        Direction::North => 0,
        Direction::East => 90,
        Direction::South => 180,
        Direction::West => 270,
    }
}
