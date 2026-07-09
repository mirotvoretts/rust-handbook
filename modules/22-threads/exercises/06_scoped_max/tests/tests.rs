use ex_22_06_scoped_max::parallel_max;

#[test]
fn max_over_several_parts() {
    assert_eq!(parallel_max(&[3, 7, 1, 9, 4, 2], 3), Some(9));
}

#[test]
fn parts_larger_than_len() {
    assert_eq!(parallel_max(&[5, 2], 8), Some(5));
}

#[test]
fn single_part_equals_plain_max() {
    assert_eq!(parallel_max(&[1, -4, 6, 6, 0], 1), Some(6));
}

#[test]
fn empty_is_none() {
    assert_eq!(parallel_max(&[], 4), None);
}

#[test]
fn all_negative() {
    assert_eq!(parallel_max(&[-9, -3, -7], 2), Some(-3));
}
