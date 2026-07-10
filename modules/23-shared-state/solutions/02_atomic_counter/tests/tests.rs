use sol_23_02_atomic_counter::atomic_increment;

#[test]
fn basic() {
    assert_eq!(atomic_increment(4, 1000), 4000);
}

#[test]
fn many_threads() {
    assert_eq!(atomic_increment(32, 500), 16000);
}
