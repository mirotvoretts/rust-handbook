//! 04 (2x) - ДОБАВЬТЕ OUTLIVES-ОГРАНИЧЕНИЕ `'big: 'a` САМИ. Эталонное решение.

pub fn as_shorter<'a, 'big: 'a>(big: &'big str) -> &'a str {
    big
}
