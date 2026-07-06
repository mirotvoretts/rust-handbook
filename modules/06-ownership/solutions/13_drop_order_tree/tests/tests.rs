use sol_06_13_drop_order_tree::Tree;
use std::cell::RefCell;
use std::rc::Rc;

// Строим дерево:
//        1
//       / \
//      2   3
//     /
//    4
fn build(log: &Rc<RefCell<Vec<u32>>>) -> Tree {
    Tree::node(
        1,
        Some(Tree::node(2, Some(Tree::leaf(4, Rc::clone(log))), None, Rc::clone(log))),
        Some(Tree::leaf(3, Rc::clone(log))),
        Rc::clone(log),
    )
}

#[test]
fn whole_subtree_freed_once_each() {
    let log = Rc::new(RefCell::new(Vec::new()));
    {
        let _t = build(&log);
    }
    let mut ids = log.borrow().clone();
    ids.sort();
    assert_eq!(ids, vec![1, 2, 3, 4]); // каждый узел освобождён ровно один раз
}

#[test]
fn drop_is_pre_order_root_then_left_then_right() {
    let log = Rc::new(RefCell::new(Vec::new()));
    {
        let _t = build(&log);
    }
    // Drop узла выполняется до Drop его полей; поля роняются в порядке объявления (left, right).
    assert_eq!(*log.borrow(), vec![1, 2, 4, 3]);
}
