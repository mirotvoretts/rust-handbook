use sol_07_06_split_slice::{halves, without_ends};

#[test]
fn splitting() {
    assert_eq!(halves(&[1, 2, 3, 4]), (&[1, 2][..], &[3, 4][..]));
    assert_eq!(halves(&[1, 2, 3]), (&[1][..], &[2, 3][..]));
    assert_eq!(halves(&[]), (&[][..], &[][..]));
}

#[test]
fn middles() {
    assert_eq!(without_ends(&[1, 2, 3, 4]), &[2, 3]);
    assert_eq!(without_ends(&[1, 2, 3]), &[2]);
    assert_eq!(without_ends(&[1, 2]), &[] as &[i32]);
    assert_eq!(without_ends(&[1]), &[] as &[i32]);
    assert_eq!(without_ends(&[]), &[] as &[i32]);
}
