//! 02 (0x) — `match`-выражение, возвращающее значения. Эталонное решение.

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub fn action(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "stop",
        TrafficLight::Yellow => "slow",
        TrafficLight::Green => "go",
    }
}

pub fn duration(light: TrafficLight) -> u32 {
    match light {
        TrafficLight::Red => 60,
        TrafficLight::Yellow => 5,
        TrafficLight::Green => 45,
    }
}

pub fn next(light: TrafficLight) -> TrafficLight {
    match light {
        TrafficLight::Green => TrafficLight::Yellow,
        TrafficLight::Yellow => TrafficLight::Red,
        TrafficLight::Red => TrafficLight::Green,
    }
}
