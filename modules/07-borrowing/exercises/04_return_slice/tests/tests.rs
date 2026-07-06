use ex_07_04_return_slice::{first_word, tail};

#[test]
fn words() {
    assert_eq!(first_word("hello world"), "hello");
    assert_eq!(first_word("hi"), "hi");
    assert_eq!(first_word(""), "");
    assert_eq!(first_word("a b c"), "a");
}

#[test]
fn tails() {
    assert_eq!(tail(&[1, 2, 3]), &[2, 3]);
    assert_eq!(tail(&[9]), &[] as &[i32]);
    assert_eq!(tail(&[]), &[] as &[i32]);
}
