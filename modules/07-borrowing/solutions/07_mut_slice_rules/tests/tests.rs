use sol_07_07_mut_slice_rules::{negate_all, swap_ends};

#[test]
fn swapping() {
    let mut a = [1, 2, 3, 4];
    swap_ends(&mut a);
    assert_eq!(a, [4, 2, 3, 1]);

    let mut b = [7];
    swap_ends(&mut b);
    assert_eq!(b, [7]);

    let mut c: [i32; 0] = [];
    swap_ends(&mut c);
    assert_eq!(c, []);
}

#[test]
fn negating() {
    let mut a = [1, -2, 3];
    negate_all(&mut a);
    assert_eq!(a, [-1, 2, -3]);
}
