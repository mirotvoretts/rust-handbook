use ex_19_03_adapters_basic::{evens, long_word_lengths, squares};

#[test]
fn maps_squares() {
    assert_eq!(squares(&[1, 2, 3, 4]), vec![1, 4, 9, 16]);
    assert_eq!(squares(&[]), Vec::<i32>::new());
}

#[test]
fn filters_evens() {
    assert_eq!(evens(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
    assert_eq!(evens(&[1, 3, 5]), Vec::<i32>::new());
}

#[test]
fn filters_then_maps() {
    let words = ["a", "bbbb", "cc", "ddddd"];
    assert_eq!(long_word_lengths(&words, 2), vec![4, 5]);
    assert_eq!(long_word_lengths(&words, 10), Vec::<usize>::new());
}
