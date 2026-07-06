use sol_07_01_shared_borrows::{count_positive, summary, total};

#[test]
fn totals() {
    assert_eq!(total(&vec![1, 2, 3]), 6);
    assert_eq!(total(&vec![]), 0);
}

#[test]
fn positives() {
    assert_eq!(count_positive(&vec![-1, 2, -3, 4]), 2);
    assert_eq!(count_positive(&vec![0, 0]), 0);
}

#[test]
fn summaries() {
    assert_eq!(summary(&vec![1, -2, 3]), (2, 2));
    let v = vec![5, 5, 5];
    assert_eq!(summary(&v), (15, 3));
    assert_eq!(v.len(), 3);
}
