use ex_33_01_array_sum::{dot, sum};

#[test]
fn sums() {
    assert_eq!(sum([1, 2, 3]), 6);
    assert_eq!(sum([10, 20, 30, 40]), 100);
    assert_eq!(sum::<0>([]), 0);
}

#[test]
fn dots() {
    assert_eq!(dot([1, 2, 3], [4, 5, 6]), 32);
    assert_eq!(dot::<0>([], []), 0);
}
