use ex_19_01_next_manual::{count_manually, drain_then_peek, first_two};

#[test]
fn takes_first_two() {
    assert_eq!(first_two(&[10, 20, 30]), (10, 20));
    assert_eq!(first_two(&[7, 8]), (7, 8));
}

#[test]
fn counts_until_none() {
    assert_eq!(count_manually(&[1, 2, 3, 4]), 4);
    assert_eq!(count_manually(&[]), 0);
}

#[test]
fn exhausted_iter_yields_none() {
    assert_eq!(drain_then_peek(&[1, 2, 3]), (3, None));
    assert_eq!(drain_then_peek(&[]), (0, None));
}
