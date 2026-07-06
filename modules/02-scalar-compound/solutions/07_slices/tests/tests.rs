use sol_02_07_slices::{first_last, max_slice, sum_slice};

#[test]
fn sums() {
    assert_eq!(sum_slice(&[]), 0);
    assert_eq!(sum_slice(&[1, 2, 3]), 6);
    let v = vec![10, 20, 30];
    assert_eq!(sum_slice(&v), 60);
}

#[test]
fn maxima() {
    assert_eq!(max_slice(&[]), None);
    assert_eq!(max_slice(&[3, 7, 2]), Some(7));
    assert_eq!(max_slice(&[-5]), Some(-5));
}

#[test]
fn ends() {
    assert_eq!(first_last(&[]), None);
    assert_eq!(first_last(&[5]), Some((5, 5)));
    assert_eq!(first_last(&[1, 2, 3]), Some((1, 3)));
}
