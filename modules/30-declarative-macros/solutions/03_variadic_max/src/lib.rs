//! 03 (1x) - `max!(a, b, ...)`: несколько правил и рекурсия. Эталонное решение.

#[macro_export]
macro_rules! max {
    ($x:expr $(,)?) => { $x };
    ($x:expr, $($rest:expr),+ $(,)?) => {{
        let head = $x;
        let tail = $crate::max!($($rest),+);
        if head >= tail { head } else { tail }
    }};
}
