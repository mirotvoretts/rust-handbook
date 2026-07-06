//! 03 (0x) - Владение через функции. Эталонное решение.

/// Забирает имя во владение и возвращает приветствие `"Привет, <name>!"`.
pub fn make_greeting(name: String) -> String {
    format!("Привет, {name}!")
}

/// Забирает строку, дописывает к ней `"!"` и возвращает её же.
pub fn append_bang(mut s: String) -> String {
    s.push('!');
    s
}

/// Забирает вектор во владение. Возвращает пару: первый элемент (если был) и остаток вектора.
pub fn split_first(mut v: Vec<i32>) -> (Option<i32>, Vec<i32>) {
    if v.is_empty() {
        (None, v)
    } else {
        let first = v.remove(0);
        (Some(first), v)
    }
}
