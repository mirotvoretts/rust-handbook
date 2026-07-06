//! 07 (2x) - Дерево на Rc с общими узлами.
//!
//! Rc позволяет одному узлу быть ребёнком нескольких родителей - структура
//! становится ориентированным ациклическим графом (DAG). Наивный обход при этом
//! ЗАХОДИТ в общий узел столько раз, сколько на него ссылок: strong_count это видит.

use std::rc::Rc;

/// Узел дерева: значение и дети (каждый - Rc, чтобы делиться между родителями).
pub struct TreeNode {
    value: i32,
    children: Vec<Rc<TreeNode>>,
}

/// Лист (узел без детей).
pub fn leaf(value: i32) -> Rc<TreeNode> {
    todo!("Rc::new(TreeNode { value, children: Vec::new() })")
}

/// Узел с детьми.
pub fn branch(value: i32, children: Vec<Rc<TreeNode>>) -> Rc<TreeNode> {
    todo!("Rc::new(TreeNode { value, children })")
}

/// Сумма значений всех узлов при обходе. Общий узел суммируется столько раз,
/// сколько раз в него зашли (двойной счёт - это ожидаемо для DAG).
pub fn sum_values(root: &Rc<TreeNode>) -> i64 {
    todo!("root.value as i64 + root.children.iter().map(sum_values).sum::<i64>()")
}

/// Число посещённых узлов при обходе (тоже с повторами для общих узлов).
pub fn count_nodes(root: &Rc<TreeNode>) -> usize {
    todo!("1 + root.children.iter().map(count_nodes).sum::<usize>()")
}
