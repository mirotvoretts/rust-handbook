//! 05 (2x) - Маркер-трейт как статическая печать. ПОЧИНИ СЛОМАННОЕ.
//!
//! Тесты НЕ КОМПИЛИРУЮТСЯ, пока вы не пометите StableSort маркером Approved.
//!
//! Approved - трейт без методов: утверждение "алгоритм прошёл проверку". deploy
//! компилируется только для помеченных типов. Пометьте StableSort (но НЕ
//! ExperimentalSort!) и реализуйте deploy_all.

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

// todo!(): пометьте StableSort маркером Approved (одна строка)

/// "Разворачивает" алгоритм: доступно только одобренным.
pub fn deploy<T: Approved + Named>(algo: &T) -> String {
    todo!("deployed: имя")
}
