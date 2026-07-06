use ex_03_06_slice_mut::{add_to_each, fill, reverse_in_place};

#[test]
fn filling() {
    let mut a = [1, 2, 3];
    fill(&mut a, 0);
    assert_eq!(a, [0, 0, 0]);
}

#[test]
fn adding() {
    let mut a = [1, 2, 3];
    add_to_each(&mut a, 10);
    assert_eq!(a, [11, 12, 13]);
}

#[test]
fn reversing() {
    let mut a = [1, 2, 3, 4];
    reverse_in_place(&mut a);
    assert_eq!(a, [4, 3, 2, 1]);

    let mut b = [7];
    reverse_in_place(&mut b);
    assert_eq!(b, [7]);

    let mut c: [i32; 0] = [];
    reverse_in_place(&mut c);
    assert_eq!(c, []);
}
