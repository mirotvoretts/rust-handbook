//! 05 (2x) - Указатели на функции. Эталонное решение.

/// Складывает.
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// Вычитает.
pub fn sub(a: i64, b: i64) -> i64 {
    a - b
}

/// Умножает.
pub fn mul(a: i64, b: i64) -> i64 {
    a * b
}

/// По имени операции - fn-указатель.
pub fn dispatch(op: &str) -> Option<fn(i64, i64) -> i64> {
    match op {
        "add" => Some(add),
        "sub" => Some(sub),
        "mul" => Some(mul),
        _ => None,
    }
}

/// Цепочка fn-указателей слева направо.
pub fn run_pipeline(start: i64, ops: &[fn(i64) -> i64]) -> i64 {
    let mut acc = start;
    for op in ops {
        acc = op(acc);
    }
    acc
}

/// Замыкание без захватов как fn-указатель.
pub fn as_fn_pointer() -> fn(i64) -> i64 {
    let p: fn(i64) -> i64 = |x| x * 2;
    p
}
