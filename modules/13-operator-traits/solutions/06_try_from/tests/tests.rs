use sol_13_06_try_from::{Age, Percent};

#[test]
fn valid_conversions() {
    assert_eq!(Age::try_from(30), Ok(Age(30)));
    assert_eq!(Age::try_from(0), Ok(Age(0)));
    assert_eq!(Age::try_from(130), Ok(Age(130)));
    assert_eq!(Percent::try_from(100), Ok(Percent(100)));
}

#[test]
fn out_of_range_fails() {
    assert!(Age::try_from(-1).is_err());
    assert!(Age::try_from(131).is_err());
    assert!(Percent::try_from(101).is_err());
    assert!(Percent::try_from(i64::MIN).is_err());
}

#[test]
fn try_into_is_free_too() {
    let age: Result<Age, _> = 25i64.try_into(); // blanket impl TryInto
    assert_eq!(age, Ok(Age(25)));
}
