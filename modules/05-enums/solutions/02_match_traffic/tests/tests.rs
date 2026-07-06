use sol_05_02_match_traffic::{action, duration, next, TrafficLight::*};

#[test]
fn actions() {
    assert_eq!(action(Red), "stop");
    assert_eq!(action(Yellow), "slow");
    assert_eq!(action(Green), "go");
}

#[test]
fn durations() {
    assert_eq!(duration(Red), 60);
    assert_eq!(duration(Yellow), 5);
    assert_eq!(duration(Green), 45);
}

#[test]
fn cycle() {
    assert_eq!(next(Green), Yellow);
    assert_eq!(next(Yellow), Red);
    assert_eq!(next(Red), Green);
}
