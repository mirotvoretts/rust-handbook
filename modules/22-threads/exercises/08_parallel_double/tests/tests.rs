use ex_22_08_parallel_double::parallel_double;

#[test]
fn doubles_all_across_parts() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    parallel_double(&mut v, 3);
    assert_eq!(v, vec![2, 4, 6, 8, 10, 12]);
}

#[test]
fn parts_larger_than_len() {
    let mut v = vec![7, 8];
    parallel_double(&mut v, 16);
    assert_eq!(v, vec![14, 16]);
}

#[test]
fn single_part() {
    let mut v = vec![-1, 0, 5];
    parallel_double(&mut v, 1);
    assert_eq!(v, vec![-2, 0, 10]);
}

#[test]
fn empty_is_noop() {
    let mut v: Vec<i32> = vec![];
    parallel_double(&mut v, 4);
    assert_eq!(v, Vec::<i32>::new());
}
