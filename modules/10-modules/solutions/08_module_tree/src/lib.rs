//! 08 (2x) — Дерево модулей и пути между соседями. Эталонное решение.

pub mod store {
    pub mod inventory {
        pub fn count() -> u32 {
            3
        }
    }

    pub mod orders {
        fn pending() -> u32 {
            2
        }

        pub fn total_needed() -> u32 {
            super::inventory::count() + pending()
        }
    }
}
