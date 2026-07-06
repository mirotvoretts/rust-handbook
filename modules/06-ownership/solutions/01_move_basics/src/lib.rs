//! 01 (0x) - Основы move-семантики. Эталонное решение.

/// Склеивает две строки в одну (владение обеими уходит внутрь и потребляется).
pub fn concat(mut a: String, b: String) -> String {
    a.push_str(&b);
    a
}

/// Забирает вектор строк во владение и возвращает суммарную длину всех строк.
pub fn total_len(items: Vec<String>) -> usize {
    items.iter().map(|s| s.len()).sum()
}

/// Забирает строку во владение и возвращает её же, но в верхнем регистре.
pub fn shout(s: String) -> String {
    s.to_uppercase()
}
