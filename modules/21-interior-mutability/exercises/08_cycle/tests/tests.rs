use ex_21_08_cycle::{next_value, node, pair, prev_value};
use std::rc::Rc;

#[test]
fn lone_node_has_no_neighbours() {
    let n = node(42);
    assert_eq!(next_value(&n), None);
    assert_eq!(prev_value(&n), None);
}

#[test]
fn pair_links_both_directions() {
    let (a, b) = pair(1, 2);
    assert_eq!(next_value(&a), Some(2)); // a -> b по сильной next
    assert_eq!(prev_value(&b), Some(1)); // b -> a по слабой prev (upgrade)
}

#[test]
fn no_strong_cycle() {
    let (a, b) = pair(1, 2);
    // a держится только переменной a: b ссылается на него СЛАБО
    assert_eq!(Rc::strong_count(&a), 1);
    assert_eq!(Rc::weak_count(&a), 1);
    // b держится переменной b + сильным a.next = 2
    assert_eq!(Rc::strong_count(&b), 2);
    assert_eq!(Rc::weak_count(&b), 0);
}

#[test]
fn dropping_head_breaks_back_link() {
    let (a, b) = pair(1, 2);
    drop(a); // сильных ссылок на первый узел не осталось
    assert_eq!(prev_value(&b), None); // upgrade слабой prev даёт None
}
