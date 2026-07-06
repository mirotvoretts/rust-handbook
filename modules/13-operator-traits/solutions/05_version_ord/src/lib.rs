//! 05 (1x) - Ord через кортежи. Эталонное решение.

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch))
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Самая свежая версия из среза (None для пустого).
pub fn newest(versions: &[Version]) -> Option<Version> {
    let mut best = *versions.first()?;
    for v in &versions[1..] {
        if *v > best {
            best = *v;
        }
    }
    Some(best)
}
