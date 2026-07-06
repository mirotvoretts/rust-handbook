use sol_03_09_inplace_transform::{prefix_sums_in_place, running_max_in_place};

#[test]
fn running_max() {
    let mut a = [3, 1, 4, 1, 5];
    running_max_in_place(&mut a);
    assert_eq!(a, [3, 3, 4, 4, 5]);

    let mut b: [i32; 0] = [];
    running_max_in_place(&mut b);
    assert_eq!(b, []);
}

#[test]
fn prefix_sums() {
    let mut a = [1, 2, 3, 4];
    prefix_sums_in_place(&mut a);
    assert_eq!(a, [1, 3, 6, 10]);

    let mut b = [5];
    prefix_sums_in_place(&mut b);
    assert_eq!(b, [5]);
}
