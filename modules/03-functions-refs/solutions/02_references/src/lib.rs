//! 02 (0x) - Ссылки `&T`: заём для чтения. Эталонное решение.

pub fn deref_sum(a: &i32, b: &i32) -> i32 {
    *a + *b
}

pub fn max_ref(a: &i32, b: &i32) -> i32 {
    if *a >= *b {
        *a
    } else {
        *b
    }
}

pub fn len_of(s: &str) -> usize {
    s.len()
}
