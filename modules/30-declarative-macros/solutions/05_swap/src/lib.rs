//! 05 (2x) - `swap!(a, b)`: обмен значений и гигиена. Эталонное решение.

#[macro_export]
macro_rules! swap {
    ($a:expr, $b:expr) => {{
        let tmp = $a;
        $a = $b;
        $b = tmp;
    }};
}
