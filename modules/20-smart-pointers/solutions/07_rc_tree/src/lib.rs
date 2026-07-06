//! 07 (2x) - Дерево на Rc с общими узлами. Эталонное решение.

use std::rc::Rc;

/// Узел дерева со значением и Rc-детьми.
pub struct TreeNode {
    value: i32,
    children: Vec<Rc<TreeNode>>,
}

/// Лист.
pub fn leaf(value: i32) -> Rc<TreeNode> {
    Rc::new(TreeNode {
        value,
        children: Vec::new(),
    })
}

/// Узел с детьми.
pub fn branch(value: i32, children: Vec<Rc<TreeNode>>) -> Rc<TreeNode> {
    Rc::new(TreeNode { value, children })
}

/// Сумма значений при обходе (общий узел учитывается многократно).
pub fn sum_values(root: &Rc<TreeNode>) -> i64 {
    root.value as i64 + root.children.iter().map(sum_values).sum::<i64>()
}

/// Число посещённых узлов при обходе.
pub fn count_nodes(root: &Rc<TreeNode>) -> usize {
    1 + root.children.iter().map(count_nodes).sum::<usize>()
}
