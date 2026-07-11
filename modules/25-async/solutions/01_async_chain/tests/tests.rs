use sol_25_01_async_chain::run_chain;

#[test]
fn basic() {
    assert_eq!(run_chain(3), 8);
}

#[test]
fn zero() {
    assert_eq!(run_chain(0), 2);
}

#[test]
fn negative() {
    assert_eq!(run_chain(-1), 0);
}

#[test]
fn larger() {
    assert_eq!(run_chain(100), 202);
}
