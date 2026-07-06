use sol_04_12_stack::Stack;

#[test]
fn empty_stack() {
    let s = Stack::new();
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
    assert_eq!(s.peek(), None);
}

#[test]
fn push_pop_lifo() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.len(), 3);
    assert!(!s.is_empty());
    assert_eq!(s.peek(), Some(3));
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
    assert!(s.is_empty());
}

#[test]
fn peek_does_not_remove() {
    let mut s = Stack::new();
    s.push(42);
    assert_eq!(s.peek(), Some(42));
    assert_eq!(s.len(), 1);
    assert_eq!(s.peek(), Some(42));
}
