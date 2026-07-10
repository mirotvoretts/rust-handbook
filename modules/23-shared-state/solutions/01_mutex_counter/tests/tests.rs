use sol_23_01_mutex_counter::concurrent_increment;

#[test]
fn basic() {
    assert_eq!(concurrent_increment(4, 1000), 4000);
}

#[test]
fn single_thread() {
    assert_eq!(concurrent_increment(1, 500), 500);
}

#[test]
fn zero_threads() {
    assert_eq!(concurrent_increment(0, 100), 0);
}

#[test]
fn many_threads() {
    assert_eq!(concurrent_increment(16, 250), 4000);
}
