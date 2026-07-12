use sol_30_07_reverse::reverse;

#[test]
fn reverses_several() {
    assert_eq!(reverse!(1, 2, 3, 4), [4, 3, 2, 1]);
}

#[test]
fn single() {
    assert_eq!(reverse!(9), [9]);
}

#[test]
fn empty() {
    let a: [i32; 0] = reverse!();
    assert_eq!(a, [] as [i32; 0]);
}

#[test]
fn evaluates_expressions() {
    assert_eq!(reverse!(1 + 1, 3, 10 / 2), [5, 3, 2]);
}
