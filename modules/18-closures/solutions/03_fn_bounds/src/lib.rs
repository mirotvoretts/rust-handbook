//! 03 (1x) - Границы Fn / FnMut / FnOnce. Эталонное решение.

/// f(f(x)) - достаточно Fn.
pub fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

/// Много вызовов + мутация окружения - FnMut.
pub fn for_each<F: FnMut(i32)>(xs: &[i32], mut f: F) {
    for &x in xs {
        f(x);
    }
}

/// Один вызов, может потребить захваты - FnOnce.
pub fn eval<R, F: FnOnce() -> R>(f: F) -> R {
    f()
}
