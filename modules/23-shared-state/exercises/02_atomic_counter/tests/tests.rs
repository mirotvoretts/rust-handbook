use ex_23_02_atomic_counter::atomic_increment;

#[test]
fn basic() {
    assert_eq!(atomic_increment(4, 1000), 4000);
}

#[test]
fn single_thread() {
    assert_eq!(atomic_increment(1, 777), 777);
}

#[test]
fn zero_work() {
    assert_eq!(atomic_increment(8, 0), 0);
}

#[test]
fn many_threads() {
    assert_eq!(atomic_increment(32, 500), 16000);
}
