//! 07 (3x) - свой глобальный аллокатор со счётчиками. Эталонное решение.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Counting;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static LIVE: AtomicUsize = AtomicUsize::new(0);
static ALLOCS: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc(layout);
        if !ptr.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::Relaxed);
            LIVE.fetch_add(layout.size(), Ordering::Relaxed);
            ALLOCS.fetch_add(1, Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        LIVE.fetch_sub(layout.size(), Ordering::Relaxed);
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// Всего байт, выделенных за время работы (только растёт).
pub fn total_allocated() -> usize {
    ALLOCATED.load(Ordering::Relaxed)
}

/// Сколько байт выделено и ещё не освобождено.
pub fn live_bytes() -> usize {
    LIVE.load(Ordering::Relaxed)
}

/// Сколько раз вызывался `alloc`.
pub fn alloc_count() -> usize {
    ALLOCS.load(Ordering::Relaxed)
}
