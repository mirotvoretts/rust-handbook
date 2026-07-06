use std::error::Error;
use ex_16_08_error_trait::{parse_records, sum_records};

#[test]
fn parses_good_input() {
    assert_eq!(parse_records(&["1", " 2 ", "3"]).unwrap(), vec![1, 2, 3]);
}

#[test]
fn reports_line_and_source() {
    let err = parse_records(&["1", "x", "3"]).unwrap_err();
    assert_eq!(err.line_no, 1);
    assert!(err.source().is_some()); // цепочка причин доступна
    assert!(!err.to_string().is_empty());
}

#[test]
fn boxed_any_error() {
    assert_eq!(sum_records(&["10", "20"]).unwrap(), 30);
    let err = sum_records(&["10", "oops"]).unwrap_err();
    assert!(err.to_string().contains('1')); // строка 1
}
