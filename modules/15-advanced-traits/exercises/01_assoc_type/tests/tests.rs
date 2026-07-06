use ex_15_01_assoc_type::{CsvParser, IntParser, Parser};

#[test]
fn int_parser() {
    assert_eq!(IntParser.parse("42"), Some(42));
    assert_eq!(IntParser.parse(" -7 "), Some(-7));
    assert_eq!(IntParser.parse("abc"), None);
}

#[test]
fn csv_parser() {
    assert_eq!(
        CsvParser.parse("a, b ,c"),
        Some(vec!["a".to_string(), "b".to_string(), "c".to_string()])
    );
    assert_eq!(CsvParser.parse(""), None);
}

// Обобщённая функция видит Output через Parser::Output.
fn parse_twice<P: Parser>(p: &P, a: &str, b: &str) -> Option<(P::Output, P::Output)> {
    Some((p.parse(a)?, p.parse(b)?))
}

#[test]
fn generic_over_parser() {
    assert_eq!(parse_twice(&IntParser, "1", "2"), Some((1, 2)));
    assert_eq!(parse_twice(&IntParser, "1", "x"), None);
}
