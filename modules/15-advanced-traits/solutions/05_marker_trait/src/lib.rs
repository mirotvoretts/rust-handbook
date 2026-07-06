//! 05 (2x) — Маркер-трейт как статическая печать. Эталонное решение.

pub trait Named {
    fn name(&self) -> &'static str;
}

/// Маркер: без методов.
pub trait Approved {}

pub struct StableSort;
pub struct ExperimentalSort;

impl Named for StableSort {
    fn name(&self) -> &'static str {
        "stable-sort"
    }
}

impl Named for ExperimentalSort {
    fn name(&self) -> &'static str {
        "experimental-sort"
    }
}

impl Approved for StableSort {}

/// «Разворачивает» алгоритм: доступно только одобренным.
pub fn deploy<T: Approved + Named>(algo: &T) -> String {
    format!("deployed: {}", algo.name())
}
