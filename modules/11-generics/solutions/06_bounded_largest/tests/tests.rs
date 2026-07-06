use sol_11_06_bounded_largest::{is_sorted, largest, smallest};

#[test]
fn ints() {
    assert_eq!(largest(&[3, 9, 4]), Some(&9));
    assert_eq!(smallest(&[3, 9, 4]), Some(&3));
}

#[test]
fn chars_and_strs_work_via_partialord() {
    assert_eq!(largest(&['a', 'z', 'm']), Some(&'z'));
    assert_eq!(smallest(&["pear", "apple", "fig"]), Some(&"apple"));
}

#[test]
fn floats_are_only_partialord_and_still_work() {
    assert_eq!(largest(&[1.5, 2.5, 0.5]), Some(&2.5));
}

#[test]
fn empty_gives_none() {
    let xs: [i32; 0] = [];
    assert_eq!(largest(&xs), None);
    assert_eq!(smallest(&xs), None);
}

#[test]
fn first_max_wins() {
    let xs = [(2, "first"), (2, "second")];
    // кортежи сравниваются лексикографически; (2,"f") < (2,"s"), максимум — second
    assert_eq!(largest(&xs), Some(&(2, "second")));
}

#[test]
fn sortedness() {
    assert!(is_sorted::<i32>(&[]));
    assert!(is_sorted(&[1]));
    assert!(is_sorted(&[1, 1, 2, 5]));
    assert!(!is_sorted(&[2, 1]));
}
