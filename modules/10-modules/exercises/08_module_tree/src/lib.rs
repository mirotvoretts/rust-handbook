//! 08 (2x) — Дерево модулей и пути между соседями.
//!
//! Дерево: `store` содержит два подмодуля-соседа — `inventory` и `orders`. Метод в `orders`
//! должен сложить данные из СОСЕДНЕГО модуля (`super::inventory::count()`) и из своего
//! приватного помощника (`pending()`). Реализуйте `total_needed`.
//!
//! Дерево модулей:
//!   crate
//!   └── store
//!       ├── inventory :: count
//!       └── orders    :: total_needed, pending (приватный)

pub mod store {
    pub mod inventory {
        /// Сколько единиц на складе.
        pub fn count() -> u32 {
            3
        }
    }

    pub mod orders {
        /// Приватный помощник: сколько заказов ожидает.
        fn pending() -> u32 {
            2
        }

        /// Итого нужно = склад (из соседнего модуля) + ожидающие заказы (свой помощник).
        pub fn total_needed() -> u32 {
            todo!("super::inventory::count() + pending()")
        }
    }
}
