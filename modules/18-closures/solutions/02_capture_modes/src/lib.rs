//! 02 (0x) - Три способа захвата. Эталонное решение.

/// Захват по &T (чтение).
pub fn peek_len(v: &[i32]) -> usize {
    let f = || v.len();
    f()
}

/// Захват по &mut T (мутация); зовём дважды.
pub fn push_twice(v: &mut Vec<i32>, x: i32) {
    let mut f = || v.push(x);
    f();
    f();
}

/// Захват по значению T через move (FnOnce).
pub fn into_owned(s: String) -> String {
    let f = move || s;
    f()
}
