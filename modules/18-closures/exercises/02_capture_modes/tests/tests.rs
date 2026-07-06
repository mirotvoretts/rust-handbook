use ex_18_02_capture_modes::{into_owned, peek_len, push_twice};

#[test]
fn read_capture() {
    assert_eq!(peek_len(&[1, 2, 3]), 3);
    assert_eq!(peek_len(&[]), 0);
}

#[test]
fn mut_capture_called_twice() {
    let mut v = vec![1];
    push_twice(&mut v, 9);
    assert_eq!(v, vec![1, 9, 9]);
}

#[test]
fn move_capture_returns_value() {
    let s = String::from("hi");
    assert_eq!(into_owned(s), "hi");
}
