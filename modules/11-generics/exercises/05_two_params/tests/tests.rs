use ex_11_05_two_params::Either;

#[test]
fn is_left_works() {
    let l: Either<i32, String> = Either::Left(1);
    let r: Either<i32, String> = Either::Right(String::from("x"));
    assert!(l.is_left());
    assert!(!r.is_left());
}

#[test]
fn into_sides() {
    let l: Either<i32, &str> = Either::Left(5);
    assert_eq!(l.into_left(), Some(5));
    let l: Either<i32, &str> = Either::Left(5);
    assert_eq!(l.into_right(), None);
    let r: Either<i32, &str> = Either::Right("hi");
    assert_eq!(r.into_right(), Some("hi"));
}

#[test]
fn swap_flips_types_and_values() {
    let l: Either<i32, &str> = Either::Left(7);
    let swapped: Either<&str, i32> = l.swap();
    assert_eq!(swapped.into_right(), Some(7));
}
