use ex_07_08_shift_by_min::shift_by_min;

#[test]
fn shifting() {
    let mut a = [3, 5, 4];
    shift_by_min(&mut a);
    assert_eq!(a, [0, 2, 1]);

    let mut b = [-1, -1];
    shift_by_min(&mut b);
    assert_eq!(b, [0, 0]);

    let mut c = [10];
    shift_by_min(&mut c);
    assert_eq!(c, [0]);

    let mut d: [i32; 0] = [];
    shift_by_min(&mut d);
    assert_eq!(d, []);
}
