//! 10 (2x) — Собрать ссылки на слова длиннее порога. Эталонное решение.

pub fn refs_longer_than<'a>(words: &[&'a str], min_len: usize) -> Vec<&'a str> {
    let mut out = Vec::new();
    for &w in words {
        if w.len() > min_len {
            out.push(w);
        }
    }
    out
}
