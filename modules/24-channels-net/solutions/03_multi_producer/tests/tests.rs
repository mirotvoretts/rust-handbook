use sol_24_03_multi_producer::parallel_sum;

#[test]
fn basic() {
    assert_eq!(parallel_sum(vec![vec![1, 2], vec![3, 4], vec![5]]), 15);
}

#[test]
fn empty_outer() {
    assert_eq!(parallel_sum(vec![]), 0);
}

#[test]
fn many_chunks() {
    let chunks: Vec<Vec<i64>> = (0..50).map(|i| vec![i, i, i]).collect();
    let expected: i64 = (0..50).map(|i| 3 * i).sum();
    assert_eq!(parallel_sum(chunks), expected);
}
