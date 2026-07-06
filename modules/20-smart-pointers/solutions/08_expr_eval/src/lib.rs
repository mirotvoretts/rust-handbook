//! 08 (3x) - Дерево выражений на Box<Expr>. Эталонное решение.

/// Узел выражения.
pub enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

use Expr::{Add, Mul, Neg, Num, Sub};

/// Лист-число.
pub fn num(n: i64) -> Box<Expr> {
    Box::new(Num(n))
}

/// Сложение.
pub fn add(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> {
    Box::new(Add(a, b))
}

/// Вычитание.
pub fn sub(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> {
    Box::new(Sub(a, b))
}

/// Умножение.
pub fn mul(a: Box<Expr>, b: Box<Expr>) -> Box<Expr> {
    Box::new(Mul(a, b))
}

/// Унарный минус.
pub fn neg(a: Box<Expr>) -> Box<Expr> {
    Box::new(Neg(a))
}

/// Рекурсивное вычисление.
pub fn eval(e: &Expr) -> i64 {
    match e {
        Num(n) => *n,
        Add(a, b) => eval(a) + eval(b),
        Sub(a, b) => eval(a) - eval(b),
        Mul(a, b) => eval(a) * eval(b),
        Neg(a) => -eval(a),
    }
}

/// Число узлов дерева.
pub fn node_count(e: &Expr) -> usize {
    match e {
        Num(_) => 1,
        Add(a, b) | Sub(a, b) | Mul(a, b) => 1 + node_count(a) + node_count(b),
        Neg(a) => 1 + node_count(a),
    }
}
