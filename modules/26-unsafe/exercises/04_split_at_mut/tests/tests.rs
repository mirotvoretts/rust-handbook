use ex_26_04_split_at_mut::split_at_mut;

#[test]
fn splits_and_allows_independent_mutation() {
    let mut data = [1, 2, 3, 4, 5];
    let (left, right) = split_at_mut(&mut data, 2);
    assert_eq!(left, &mut [1, 2]);
    assert_eq!(right, &mut [3, 4, 5]);
    left[0] = 100;
    right[2] = 500;
    assert_eq!(data, [100, 2, 3, 4, 500]);
}

#[test]
fn split_at_zero() {
    let mut data = [7, 8, 9];
    let (left, right) = split_at_mut(&mut data, 0);
    assert!(left.is_empty());
    assert_eq!(right, &mut [7, 8, 9]);
}

#[test]
fn split_at_len() {
    let mut data = [7, 8, 9];
    let (left, right) = split_at_mut(&mut data, 3);
    assert_eq!(left, &mut [7, 8, 9]);
    assert!(right.is_empty());
}

#[test]
#[should_panic]
fn mid_beyond_len_panics() {
    let mut data = [1, 2, 3];
    let _ = split_at_mut(&mut data, 4);
}
