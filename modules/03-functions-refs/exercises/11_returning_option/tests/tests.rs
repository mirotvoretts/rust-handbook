use ex_03_11_returning_option::{find_index, nth, safe_div};

#[test]
fn division() {
    assert_eq!(safe_div(10, 2), Some(5));
    assert_eq!(safe_div(7, 3), Some(2));
    assert_eq!(safe_div(1, 0), None);
}

#[test]
fn indexing() {
    let xs = [10, 20, 30];
    assert_eq!(nth(&xs, 0), Some(10));
    assert_eq!(nth(&xs, 2), Some(30));
    assert_eq!(nth(&xs, 3), None);
    assert_eq!(nth(&[], 0), None);
}

#[test]
fn finding() {
    let xs = [4, 5, 6, 5];
    assert_eq!(find_index(&xs, 5), Some(1));
    assert_eq!(find_index(&xs, 4), Some(0));
    assert_eq!(find_index(&xs, 9), None);
}
