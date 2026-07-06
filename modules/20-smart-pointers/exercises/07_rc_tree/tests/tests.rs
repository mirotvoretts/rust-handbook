use ex_20_07_rc_tree::{branch, count_nodes, leaf, sum_values};
use std::rc::Rc;

// Строим DAG с общим листом d:
//   a(1) -> b(2) -> d(4)
//        -> c(3) -> d(4)   (тот же d)
fn build() -> (Rc<ex_20_07_rc_tree::TreeNode>, Rc<ex_20_07_rc_tree::TreeNode>) {
    let d = leaf(4);
    let b = branch(2, vec![Rc::clone(&d)]);
    let c = branch(3, vec![Rc::clone(&d)]);
    let a = branch(1, vec![b, c]);
    (a, d)
}

#[test]
fn shared_node_has_multiple_refs() {
    let (_a, d) = build();
    // на d ссылаются: сам d + клон в b + клон в c = 3
    assert_eq!(Rc::strong_count(&d), 3);
}

#[test]
fn traversal_double_counts_shared() {
    let (a, _d) = build();
    // сумма: 1 + 2 + 4 + 3 + 4 = 14 (лист d учтён дважды)
    assert_eq!(sum_values(&a), 14);
    // узлов при обходе: a, b, d, c, d = 5
    assert_eq!(count_nodes(&a), 5);
}

#[test]
fn single_leaf() {
    let l = leaf(7);
    assert_eq!(sum_values(&l), 7);
    assert_eq!(count_nodes(&l), 1);
    assert_eq!(Rc::strong_count(&l), 1);
}
