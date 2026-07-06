use sol_16_03_question_mark::{mean_of_two, parse_and_add};

#[test]
fn adds() {
    assert_eq!(parse_and_add("2", " 3 "), Ok(5));
}

#[test]
fn propagates_first_error() {
    assert!(parse_and_add("x", "3").is_err());
    assert!(parse_and_add("2", "y").is_err());
}

#[test]
fn mean() {
    assert_eq!(mean_of_two("10", "20"), Ok(15));
    assert!(mean_of_two("a", "b").is_err());
}
