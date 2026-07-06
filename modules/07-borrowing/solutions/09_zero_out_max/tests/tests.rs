use sol_07_09_zero_out_max::zero_out_max;

#[test]
fn zeroing() {
    let mut a = [1, 9, 3, 9];
    zero_out_max(&mut a);
    assert_eq!(a, [1, 0, 3, 9]);

    let mut b = [5];
    zero_out_max(&mut b);
    assert_eq!(b, [0]);

    let mut c = [-3, -1, -2];
    zero_out_max(&mut c);
    assert_eq!(c, [-3, 0, -2]);

    let mut d: [i32; 0] = [];
    zero_out_max(&mut d);
    assert_eq!(d, []);
}
