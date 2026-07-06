use ex_11_03_turbofish::{one_to_n, parse_all, sum_parsed};

#[test]
fn parses_good_skips_bad() {
    assert_eq!(parse_all(&["1", "x", "-3", "2.5", "42"]), vec![1, -3, 42]);
}

#[test]
fn range_collect() {
    assert_eq!(one_to_n(5), vec![1, 2, 3, 4, 5]);
    assert_eq!(one_to_n(0), Vec::<u32>::new());
}

#[test]
fn sums() {
    assert_eq!(sum_parsed(&["10", "oops", "-4"]), 6);
    assert_eq!(sum_parsed(&[]), 0);
    assert_eq!(sum_parsed(&["2147483647", "1"]), 2147483648); // не переполняется: i64
}
