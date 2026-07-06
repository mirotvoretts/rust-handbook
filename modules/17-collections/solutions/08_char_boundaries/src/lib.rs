//! 08 (2x) — Срезы по границам символов. Эталонное решение.

/// Обрезает строку до max_bytes БАЙТ, не разрывая символы; добавляет "…" если резали.
pub fn truncate_bytes(s: &str, max_bytes: usize) -> String {
    if s.len() <= max_bytes {
        return s.to_string();
    }
    let mut cut = 0;
    for (i, ch) in s.char_indices() {
        if i + ch.len_utf8() > max_bytes {
            break;
        }
        cut = i + ch.len_utf8();
    }
    format!("{}…", &s[..cut])
}

/// Байтовое смещение начала n-го символа (None, если символов меньше).
pub fn char_offset(s: &str, n: usize) -> Option<usize> {
    s.char_indices().nth(n).map(|(i, _)| i)
}
