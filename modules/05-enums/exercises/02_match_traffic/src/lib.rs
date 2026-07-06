//! 02 (0x) - `match`-выражение, возвращающее значения.
//!
//! `match` - это выражение: его результат можно вернуть напрямую. Разберите светофор.

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

/// Что делать на этот сигнал: Red -> "stop", Yellow -> "slow", Green -> "go".
pub fn action(light: TrafficLight) -> &'static str {
    todo!()
}

/// Длительность сигнала в секундах: Red 60, Yellow 5, Green 45.
pub fn duration(light: TrafficLight) -> u32 {
    todo!()
}

/// Следующий сигнал в цикле: Green -> Yellow -> Red -> Green.
pub fn next(light: TrafficLight) -> TrafficLight {
    todo!()
}
