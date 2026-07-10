use ex_24_01_channel_sum::sum_over_channel;

#[test]
fn basic() {
    assert_eq!(sum_over_channel(vec![1, 2, 3, 4]), 10);
}

#[test]
fn empty() {
    assert_eq!(sum_over_channel(vec![]), 0);
}

#[test]
fn negatives() {
    assert_eq!(sum_over_channel(vec![-5, 10, -3]), 2);
}

#[test]
fn larger() {
    let v: Vec<i64> = (1..=1000).collect();
    assert_eq!(sum_over_channel(v), 500_500);
}
