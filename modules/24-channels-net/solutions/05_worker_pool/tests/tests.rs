use sol_24_05_worker_pool::pool_sum_of_squares;

#[test]
fn small() {
    assert_eq!(pool_sum_of_squares(vec![1, 2, 3, 4], 2), 30);
}

#[test]
fn one_worker() {
    assert_eq!(pool_sum_of_squares(vec![1, 2, 3, 4], 1), 30);
}

#[test]
fn no_jobs() {
    assert_eq!(pool_sum_of_squares(vec![], 4), 0);
}

#[test]
fn more_workers_than_jobs() {
    assert_eq!(pool_sum_of_squares(vec![5, 6], 8), 25 + 36);
}

#[test]
fn larger() {
    let jobs: Vec<u64> = (1..=100).collect();
    let expected: u64 = (1..=100).map(|n| n * n).sum();
    assert_eq!(pool_sum_of_squares(jobs, 4), expected);
}
