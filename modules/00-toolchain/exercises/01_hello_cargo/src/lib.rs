//! 01 (0x) - Первый запуск: убедитесь, что цикл `cargo test` работает.
//!
//! Замените `todo!()` так, чтобы функция возвращала строку "Hello, cargo!".
//! Запуск: `cargo test -p ex-00-01-hello-cargo`.

/// Возвращает приветствие "Hello, cargo!".
pub fn greeting() -> String {
    todo!("верните String::from(\"Hello, cargo!\")")
}
