use ex_02_05_tuples::{divmod, min_max, swap_pair};

#[test]
fn division() {
    assert_eq!(divmod(17, 5), (3, 2));
    assert_eq!(divmod(10, 2), (5, 0));
}

#[test]
fn extremes() {
    assert_eq!(min_max(3, 1, 2), (1, 3));
    assert_eq!(min_max(5, 5, 5), (5, 5));
    assert_eq!(min_max(-1, 4, 0), (-1, 4));
}

#[test]
fn swapping() {
    assert_eq!(swap_pair((1, 2)), (2, 1));
    assert_eq!(swap_pair((0, -7)), (-7, 0));
}
