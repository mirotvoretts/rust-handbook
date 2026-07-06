use ex_20_03_recursive_list::{from_slice, len, sum, List};

#[test]
fn builds_and_measures() {
    let list = from_slice(&[1, 2, 3, 4]);
    assert_eq!(len(&list), 4);
    assert_eq!(sum(&list), 10);
}

#[test]
fn empty_list() {
    let list = from_slice(&[]);
    assert_eq!(len(&list), 0);
    assert_eq!(sum(&list), 0);
    assert!(matches!(list, List::Nil));
}

#[test]
fn preserves_order_head() {
    let list = from_slice(&[10, 20, 30]);
    // голова - первый элемент
    match list {
        List::Cons(v, _) => assert_eq!(v, 10),
        List::Nil => panic!("ожидался Cons"),
    }
}

#[test]
fn single_element() {
    let list = from_slice(&[42]);
    assert_eq!(len(&list), 1);
    assert_eq!(sum(&list), 42);
}
