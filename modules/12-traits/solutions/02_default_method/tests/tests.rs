use sol_12_02_default_method::{Greet, Pirate, Robot};

#[test]
fn robot_uses_default() {
    assert_eq!(Robot.greet(), "Привет, R2!");
}

#[test]
fn pirate_overrides() {
    assert_eq!(Pirate.greet(), "Йо-хо-хо, Флинт!");
}

#[test]
fn names() {
    assert_eq!(Robot.name(), "R2");
    assert_eq!(Pirate.name(), "Флинт");
}
