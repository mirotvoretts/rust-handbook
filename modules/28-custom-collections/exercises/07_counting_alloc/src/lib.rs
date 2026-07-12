//! 07 (3x) - свой глобальный аллокатор со счётчиками (раздел 7 README).
//!
//! Аллокатор, который используется всей программой, - это тип, реализующий трейт
//! `GlobalAlloc` и помеченный атрибутом `#[global_allocator]`. Здесь мы не пишем
//! собственное управление памятью, а оборачиваем системный аллокатор `System`,
//! пропуская вызовы через себя и попутно считая байты. Такой приём применяют для
//! профилирования: узнать, сколько и как часто программа выделяет.
//!
//! Каркас уже даёт рабочий "прозрачный" аллокатор (forward в `System`) и три статических
//! счётчика (`AtomicUsize`). Аллокатор обязан оставаться рабочим на каждом шаге -
//! `todo!()` в теле `alloc` уронил бы программу ещё до `main`, поэтому реализуй не тела
//! `alloc`/`dealloc` целиком, а *учёт* внутри них и геттеры.
//!
//! Что сделать:
//! - В `alloc`: после успешного `System.alloc` (когда указатель не null) прибавить
//!   `layout.size()` к `ALLOCATED` (всего выделено за жизнь) и к `LIVE` (сейчас занято),
//!   увеличить `ALLOCS` на 1. Порядок памяти - `Ordering::Relaxed` (счётчики независимы,
//!   нам важен только сам факт инкремента, а не синхронизация с другими данными).
//! - В `dealloc`: после `System.dealloc` вычесть `layout.size()` из `LIVE`
//!   (`fetch_sub`). `ALLOCATED` не трогаем - это накопительный итог.
//! - Реализуй геттеры `total_allocated`, `live_bytes`, `alloc_count` (через `.load`).
//!
//! Почему учитывать только при не-null указателе: если `System.alloc` вернул null
//! (выделение не удалось), никакой памяти не появилось - счётчик врать не должен.
//!
//! `GlobalAlloc`: <https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html>.
//! `#[global_allocator]`: <https://doc.rust-lang.org/std/alloc/index.html#the-global_allocator-attribute>.
//! Атомарные счётчики (`fetch_add`/`fetch_sub`/`load`, `Ordering`) разбирались в M23:
//! <https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html>.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::AtomicUsize;

pub struct Counting;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static LIVE: AtomicUsize = AtomicUsize::new(0);
static ALLOCS: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc(layout);
        // TODO: если ptr не null, учесть layout.size() в ALLOCATED и LIVE, увеличить ALLOCS.
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        // TODO: вычесть layout.size() из LIVE.
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// Всего байт, выделенных за время работы (только растёт).
pub fn total_allocated() -> usize {
    todo!()
}

/// Сколько байт выделено и ещё не освобождено.
pub fn live_bytes() -> usize {
    todo!()
}

/// Сколько раз вызывался `alloc`.
pub fn alloc_count() -> usize {
    todo!()
}
