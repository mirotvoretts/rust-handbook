use ex_30_03_variadic_max::max;

#[test]
fn single() {
    assert_eq!(max!(5), 5);
}

#[test]
fn several() {
    assert_eq!(max!(3, 9, 2, 7), 9);
    assert_eq!(max!(1, 2), 2);
}

#[test]
fn negatives() {
    assert_eq!(max!(-1, -5, -3), -1);
}

#[test]
fn trailing_comma() {
    assert_eq!(max!(4, 8, 6,), 8);
}
