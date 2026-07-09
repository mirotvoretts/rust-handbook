use ex_21_05_shared_list::{add_all, from_slice, sum};

#[test]
fn build_and_sum() {
    let head = from_slice(&[1, 2, 3]);
    assert_eq!(sum(&head), 6);
}

#[test]
fn empty_list() {
    let head = from_slice(&[]);
    assert!(head.is_none());
    assert_eq!(sum(&head), 0);
}

#[test]
fn add_all_mutates_every_node() {
    let head = from_slice(&[1, 2, 3]);
    add_all(&head, 10);
    assert_eq!(sum(&head), 36); // 11 + 12 + 13
}

#[test]
fn mutation_through_shared_handle_is_visible() {
    let head = from_slice(&[1, 2, 3]).unwrap();
    // вторая ручка на второй узел
    let second = head.borrow().next.clone().unwrap();
    second.borrow_mut().value = 99;
    // видно через голову: голова и second делят один Rc второго узла
    let via_head = head.borrow().next.as_ref().unwrap().borrow().value;
    assert_eq!(via_head, 99);
}
