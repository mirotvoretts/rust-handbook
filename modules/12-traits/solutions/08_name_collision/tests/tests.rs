use sol_12_08_name_collision::{fly_as_pilot, fly_as_wizard, Human};

#[test]
fn inherent_method_wins_on_dot_call() {
    assert_eq!(Human.fly(), "машу руками");
}

#[test]
fn trait_methods_via_dispatchers() {
    assert_eq!(fly_as_pilot(&Human), "выполняю рейс");
    assert_eq!(fly_as_wizard(&Human), "лечу на метле");
}

#[test]
fn fully_qualified_from_test_side() {
    use sol_12_08_name_collision::{Pilot, Wizard};
    assert_eq!(Pilot::fly(&Human), "выполняю рейс");
    assert_eq!(<Human as Wizard>::fly(&Human), "лечу на метле");
}
