//! 05 (1x) — Ord через кортежи.
//!
//! Семантическая версия сравнивается лексикографически: сначала major, при равенстве
//! minor, затем patch. Кортежи в Rust сравниваются именно так — делегируйте им.
//! PartialOrd выражайте через cmp (Some(self.cmp(other))) — это стандартная идиома.

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!("сравните кортежи (major, minor, patch)")
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!("делегируйте к cmp")
    }
}

/// Самая свежая версия из среза (None для пустого).
pub fn newest(versions: &[Version]) -> Option<Version> {
    todo!("теперь есть Ord: сравнивайте напрямую > <")
}
