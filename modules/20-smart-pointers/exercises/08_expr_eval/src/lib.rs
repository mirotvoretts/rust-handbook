//! 08 (3x) - Капстоун: дерево выражений на Box<Expr>.
//!
//! Арифметическое выражение - рекурсивный тип: узел содержит поддеревья. Как и
//! cons-список, без Box он имел бы бесконечный размер. Конструкторы прячут
//! Box::new, а eval рекурсивно сворачивает дерево в число.

/// Узел выражения.
pub enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

/// Лист-число.
pub fn num(n: i64) -> Box<Expr> {
    todo!("Box::new(Expr::Num(n))")
}

/// Сложение.
pub fn add(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> {
    todo!("Box::new(Expr::Add(a, b))")
}

/// Вычитание.
pub fn sub(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> {
    todo!("Box::new(Expr::Sub(a, b))")
}

/// Умножение.
pub fn mul(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> {
    todo!("Box::new(Expr::Mul(a, b))")
}

/// Унарный минус.
pub fn neg(a: Box<Expr>) -> Box<Expr> {
    todo!("Box::new(Expr::Neg(a))")
}

/// Вычислить выражение. Рекурсивно спускайся по поддеревьям.
/// Подсказка: eval(a) с a: &Box<Expr> работает - &Box<Expr> коерсится в &Expr.
pub fn eval(e: &Expr) -> i64 {
    todo!("match e { Num(n) => *n, Add(a, b) => eval(a) + eval(b), ... , Neg(a) => -eval(a) }")
}

/// Число узлов в дереве (включая числа и операции).
pub fn node_count(e: &Expr) -> usize {
    todo!("Num => 1; бинарные => 1 + count(a) + count(b); Neg => 1 + count(a)")
}
