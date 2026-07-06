use ex_16_06_let_else::{extract_count, sum_first_two};

#[test]
fn extracts_count() {
    assert_eq!(extract_count("count: 42; rest"), "count=42");
    assert_eq!(extract_count("count: 7"), "count=7");
}

#[test]
fn missing_or_bad_count() {
    assert_eq!(extract_count("total: 42"), "n/a");
    assert_eq!(extract_count("count: xyz"), "n/a");
}

#[test]
fn sums() {
    assert_eq!(sum_first_two(&[3, 4, 100]), 7);
    assert_eq!(sum_first_two(&[3]), 0);
    assert_eq!(sum_first_two(&[]), 0);
}
