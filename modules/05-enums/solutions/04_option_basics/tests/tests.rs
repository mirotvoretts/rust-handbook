use sol_05_04_option_basics::{describe, first, safe_sqrt};

#[test]
fn sqrt() {
    assert_eq!(safe_sqrt(-1.0), None);
    match safe_sqrt(9.0) {
        Some(v) => assert!((v - 3.0).abs() < 1e-9),
        None => panic!("ожидался Some"),
    }
}

#[test]
fn firsts() {
    assert_eq!(first(&[10, 20]), Some(10));
    assert_eq!(first(&[]), None);
}

#[test]
fn descriptions() {
    assert_eq!(describe(Some(7)), "got 7");
    assert_eq!(describe(None), "nothing");
}
