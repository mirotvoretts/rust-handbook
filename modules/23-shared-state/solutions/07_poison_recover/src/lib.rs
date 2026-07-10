//! 07 (3x) - Poisoning: чтение сквозь отравленный замок. Эталонное решение.

use std::sync::Mutex;

pub fn recover_sum(m: &Mutex<Vec<i64>>) -> i64 {
    let guard = m.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    guard.iter().sum()
}
