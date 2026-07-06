use ex_08_03_longer_slice::longer_slice;

#[test]
fn picks_longer() {
    assert_eq!(longer_slice(&[1, 2, 3], &[9]), &[1, 2, 3]);
    assert_eq!(longer_slice(&[1], &[9, 8]), &[9, 8]);
}

#[test]
fn tie_returns_first() {
    assert_eq!(longer_slice(&[1, 2], &[3, 4]), &[1, 2]);
}
