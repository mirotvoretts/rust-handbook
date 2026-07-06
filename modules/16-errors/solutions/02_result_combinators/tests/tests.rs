use sol_16_02_result_combinators::{double_ok, tag_error, to_option, value_or_err_len};

#[test]
fn map_only_touches_ok() {
    assert_eq!(double_ok(Ok(21)), Ok(42));
    assert_eq!(double_ok(Err(String::from("x"))), Err(String::from("x")));
}

#[test]
fn map_err_only_touches_err() {
    assert_eq!(tag_error(Ok(1)), Ok(1));
    assert_eq!(tag_error(Err(String::from("boom"))), Err(String::from("error: boom")));
}

#[test]
fn ok_drops_error() {
    assert_eq!(to_option(Ok(7)), Some(7));
    assert_eq!(to_option(Err(String::from("nope"))), None);
}

#[test]
fn lazy_fallback_from_error() {
    assert_eq!(value_or_err_len(Ok(100)), 100);
    assert_eq!(value_or_err_len(Err(String::from("abc"))), 3);
}
