use ex_22_02_move_capture::shout_in_thread;

#[test]
fn uppercases_owned_string() {
    assert_eq!(shout_in_thread(String::from("rust")), "RUST");
}

#[test]
fn empty_string() {
    assert_eq!(shout_in_thread(String::new()), "");
}

#[test]
fn already_upper() {
    assert_eq!(shout_in_thread(String::from("Go!")), "GO!");
}
