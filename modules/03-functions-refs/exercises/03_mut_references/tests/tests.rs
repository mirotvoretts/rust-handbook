use ex_03_03_mut_references::{double, increment, set_to};

#[test]
fn mutation() {
    let mut x = 5;
    increment(&mut x);
    assert_eq!(x, 6);
    double(&mut x);
    assert_eq!(x, 12);
    set_to(&mut x, 0);
    assert_eq!(x, 0);
}

#[test]
fn independent_calls() {
    let mut a = 10;
    let mut b = -3;
    increment(&mut a);
    double(&mut b);
    assert_eq!(a, 11);
    assert_eq!(b, -6);
}
