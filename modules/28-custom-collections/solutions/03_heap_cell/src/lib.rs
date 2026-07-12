//! 03 (1x) - выделить одно значение на куче вручную. Эталонное решение.

use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::ptr::NonNull;

/// Выделить ячейку `u64` на куче и записать в неё `v`.
pub fn alloc_u64(v: u64) -> NonNull<u64> {
    let layout = Layout::new::<u64>();
    // SAFETY: layout ненулевого размера (u64 - 8 байт).
    let raw = unsafe { alloc(layout) } as *mut u64;
    let ptr = match NonNull::new(raw) {
        Some(p) => p,
        None => handle_alloc_error(layout),
    };
    // SAFETY: память свежая и выровнена под u64; write инициализирует её без drop старого.
    unsafe { ptr.as_ptr().write(v) };
    ptr
}

/// # Safety
/// `p` получен из `alloc_u64` и ещё не освобождён.
pub unsafe fn read_u64(p: NonNull<u64>) -> u64 {
    p.as_ptr().read()
}

/// # Safety
/// `p` получен из `alloc_u64`, вызывается ровно один раз, дальше `p` не используется.
pub unsafe fn free_u64(p: NonNull<u64>) {
    dealloc(p.as_ptr() as *mut u8, Layout::new::<u64>());
}
