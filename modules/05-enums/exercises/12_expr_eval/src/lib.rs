//! 12 (3x) — Рекурсивный enum-выражение и его вычисление.
//!
//! `Expr` — дерево арифметического выражения. Варианты, содержащие подвыражения, заворачивают
//! их в `Box` (иначе тип был бы бесконечного размера). Вычислите значение рекурсивным `match`.

pub enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

/// Вычисляет значение выражения.
///   Num(n)    -> n
///   Add(a, b) -> eval(a) + eval(b)
///   Mul(a, b) -> eval(a) * eval(b)
///   Neg(a)    -> -eval(a)
pub fn eval(expr: &Expr) -> i64 {
    todo!("рекурсивный match по вариантам Expr")
}
