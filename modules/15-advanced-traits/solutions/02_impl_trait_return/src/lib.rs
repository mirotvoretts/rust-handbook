//! 02 (1x) - impl Trait в возвращаемой позиции. Эталонное решение.

/// Возвращает функцию "прибавить k".
pub fn make_adder(k: i32) -> impl Fn(i32) -> i32 {
    move |x| x + k
}

/// Возвращает функцию "умножить на k".
pub fn make_scaler(k: i32) -> impl Fn(i32) -> i32 {
    move |x| x * k
}

/// Композиция: сначала f, потом g.
pub fn compose(
    f: impl Fn(i32) -> i32,
    g: impl Fn(i32) -> i32,
) -> impl Fn(i32) -> i32 {
    move |x| g(f(x))
}
