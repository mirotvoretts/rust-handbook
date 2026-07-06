//! 07 (1x) - Перемещение значения из-под `&mut`. Эталонное решение.

use std::mem;

pub fn take_out(slot: &mut Option<String>) -> Option<String> {
    slot.take()
}

pub fn replace_with(slot: &mut String, new: String) -> String {
    mem::replace(slot, new)
}

pub fn drain_vec(slot: &mut Vec<i32>) -> Vec<i32> {
    mem::take(slot)
}
