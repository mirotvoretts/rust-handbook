use sol_26_03_sum_raw::sum_raw;

#[test]
fn basic() {
    let v = vec![1, 2, 3, 4];
    assert_eq!(unsafe { sum_raw(v.as_ptr(), v.len()) }, 10);
}

#[test]
fn empty() {
    let v: Vec<i64> = Vec::new();
    assert_eq!(unsafe { sum_raw(v.as_ptr(), v.len()) }, 0);
}

#[test]
fn negatives() {
    let v = vec![-5, 10, -3];
    assert_eq!(unsafe { sum_raw(v.as_ptr(), v.len()) }, 2);
}

#[test]
fn larger() {
    let v: Vec<i64> = (1..=1000).collect();
    assert_eq!(unsafe { sum_raw(v.as_ptr(), v.len()) }, 500_500);
}
