use sol_11_10_generic_stack::Stack;

#[test]
fn push_pop_lifo() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
}

#[test]
fn peek_does_not_remove() {
    let mut s = Stack::new();
    s.push("top");
    assert_eq!(s.peek(), Some(&"top"));
    assert_eq!(s.len(), 1);
}

#[test]
fn peek_mut_allows_mutation() {
    let mut s = Stack::new();
    s.push(String::from("hell"));
    if let Some(top) = s.peek_mut() {
        top.push('o');
    }
    assert_eq!(s.pop().as_deref(), Some("hello"));
}

#[test]
fn len_and_empty() {
    let mut s: Stack<i32> = Stack::new();
    assert!(s.is_empty());
    s.push(5);
    assert_eq!(s.len(), 1);
    assert!(!s.is_empty());
}

fn stringify(n: i32) -> String { format!("#{n}") }

#[test]
fn map_preserves_order_changes_type() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    let mut t: Stack<String> = s.map(stringify);
    assert_eq!(t.pop().as_deref(), Some("#2"));
    assert_eq!(t.pop().as_deref(), Some("#1"));
}
