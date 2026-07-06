use sol_05_11_parse_and_add::parse_and_add;

#[test]
fn both_ok() {
    assert_eq!(parse_and_add("2", "3"), Ok(5));
    assert_eq!(parse_and_add("-10", "4"), Ok(-6));
}

#[test]
fn first_bad() {
    assert_eq!(parse_and_add("x", "3"), Err(String::from("bad number: x")));
}

#[test]
fn second_bad() {
    assert_eq!(parse_and_add("2", "y"), Err(String::from("bad number: y")));
}
