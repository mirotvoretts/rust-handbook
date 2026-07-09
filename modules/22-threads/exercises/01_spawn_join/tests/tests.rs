use ex_22_01_spawn_join::spawn_add;

#[test]
fn adds_in_thread() {
    assert_eq!(spawn_add(2, 2), 4);
}

#[test]
fn handles_negatives() {
    assert_eq!(spawn_add(-5, 3), -2);
}

#[test]
fn zero() {
    assert_eq!(spawn_add(0, 0), 0);
}
