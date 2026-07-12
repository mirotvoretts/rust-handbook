use ex_29_08_callback_userdata::*;

#[test]
fn counts_target() {
    let events = [1, 2, 2, 3, 2, 1];
    assert_eq!(count_matching(&events, 2), 3);
    assert_eq!(count_matching(&events, 1), 2);
    assert_eq!(count_matching(&events, 3), 1);
}

#[test]
fn no_match() {
    let events = [1, 2, 3];
    assert_eq!(count_matching(&events, 9), 0);
}

#[test]
fn empty_events() {
    assert_eq!(count_matching(&[], 1), 0);
}
