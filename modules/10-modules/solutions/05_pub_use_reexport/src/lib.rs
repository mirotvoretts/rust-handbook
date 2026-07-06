//! 05 (1x) - Ре-экспорт через `pub use` (фасад). Эталонное решение.

mod internal {
    pub fn compute(n: i32) -> i32 {
        n + 100
    }
}

pub use internal::compute;
