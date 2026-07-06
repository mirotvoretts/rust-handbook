use ex_07_11_dedup_sorted::dedup_sorted;

#[test]
fn dedup() {
    let mut a = vec![1, 1, 2, 3, 3, 3];
    dedup_sorted(&mut a);
    assert_eq!(a, vec![1, 2, 3]);

    let mut b = vec![1, 2, 3];
    dedup_sorted(&mut b);
    assert_eq!(b, vec![1, 2, 3]);

    let mut c = vec![5, 5, 5];
    dedup_sorted(&mut c);
    assert_eq!(c, vec![5]);

    let mut d: Vec<i32> = Vec::new();
    dedup_sorted(&mut d);
    assert_eq!(d, Vec::<i32>::new());
}
