use ex_17_01_vec_toolbox::{append_all, keep_even, squash, squares};

#[test]
fn retain_evens() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    keep_even(&mut v);
    assert_eq!(v, vec![2, 4, 6]);
}

#[test]
fn extend_strings() {
    let mut v = vec![String::from("a")];
    append_all(&mut v, &["b", "c"]);
    assert_eq!(v, vec!["a", "b", "c"]);
}

#[test]
fn dedup_adjacent_only() {
    let mut v = vec![1, 1, 2, 2, 2, 1];
    squash(&mut v);
    assert_eq!(v, vec![1, 2, 1]); // последняя 1 НЕ соседняя — остаётся
}

#[test]
fn squares_no_realloc() {
    let v = squares(5);
    assert_eq!(v, vec![1, 4, 9, 16, 25]);
    assert!(v.capacity() >= 5 && v.capacity() <= 8, "cap={}", v.capacity());
    assert_eq!(squares(0), Vec::<u64>::new());
}
