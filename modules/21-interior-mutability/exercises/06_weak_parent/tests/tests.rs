use ex_21_06_weak_parent::{add_child, leaf, parent_value};
use std::rc::Rc;

#[test]
fn child_sees_parent() {
    let root = leaf(1);
    let child = leaf(2);
    add_child(&root, &child);
    assert_eq!(parent_value(&child), Some(1));
}

#[test]
fn root_has_no_parent() {
    let root = leaf(1);
    assert_eq!(parent_value(&root), None);
}

#[test]
fn counts_show_no_strong_cycle() {
    let root = leaf(1);
    let child = leaf(2);
    add_child(&root, &child);
    // root держится только переменной: ребёнок ссылается на него СЛАБО
    assert_eq!(Rc::strong_count(&root), 1);
    assert_eq!(Rc::weak_count(&root), 1);
    // child держится переменной child + вектором root.children = 2 сильных
    assert_eq!(Rc::strong_count(&child), 2);
}

#[test]
fn dead_parent_upgrades_to_none() {
    let child = leaf(2);
    {
        let root = leaf(1);
        add_child(&root, &child);
        assert_eq!(parent_value(&child), Some(1));
        // root уходит из области видимости здесь
    }
    // сильных ссылок на root не осталось -> upgrade даёт None
    assert_eq!(parent_value(&child), None);
}
