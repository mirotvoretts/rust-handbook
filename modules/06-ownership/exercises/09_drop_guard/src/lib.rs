//! 09 (2x) - RAII-гвард с побочным эффектом на `Drop`.
//!
//! Классический приём: объект захватывает ресурс при создании и освобождает его в `Drop`.
//! Так ресурс гарантированно освобождается ровно один раз - на выходе из области видимости
//! или при досрочном `drop()`. Это Rust-версия C++-идиомы RAII (lock_guard, scope guard).
//!
//! `Transaction` при создании пишет в журнал `"begin <name>"`, а при уничтожении - `"end <name>"`.
//! Реализуйте `begin` и `Drop`.

use std::cell::RefCell;
use std::rc::Rc;

pub struct Transaction {
    name: String,
    log: Rc<RefCell<Vec<String>>>,
}

impl Transaction {
    /// Начинает транзакцию: немедленно пишет в журнал `format!("begin {name}")`.
    pub fn begin(name: &str, log: Rc<RefCell<Vec<String>>>) -> Transaction {
        todo!()
    }
}

// TODO: реализуйте `impl Drop for Transaction`, дописывающий в журнал `format!("end {name}")`.
