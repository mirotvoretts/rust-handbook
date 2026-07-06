//! 02 (0x) - Три формы обхода. Эталонное решение.

/// iter(): читаем.
pub fn sum_borrowed(v: &[i64]) -> i64 {
    let mut sum = 0;
    for x in v {
        sum += *x;
    }
    sum
}

/// iter_mut(): мутируем на месте.
pub fn double_in_place(v: &mut [i64]) {
    for x in v.iter_mut() {
        *x *= 2;
    }
}

/// into_iter(): потребляем.
pub fn concat_owned(v: Vec<String>) -> String {
    let mut out = String::new();
    for s in v {
        out.push_str(&s);
    }
    out
}
