use sol_05_08_result_basics::{checked_div, parse_bool};

#[test]
fn division() {
    assert_eq!(checked_div(10, 2), Ok(5));
    assert_eq!(checked_div(1, 0), Err(String::from("cannot divide by zero")));
}

#[test]
fn booleans() {
    assert_eq!(parse_bool("true"), Ok(true));
    assert_eq!(parse_bool("false"), Ok(false));
    assert_eq!(parse_bool("yes"), Err(String::from("invalid bool: yes")));
}
