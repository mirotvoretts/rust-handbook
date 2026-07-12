//! 07 (3x) - вызов libc `qsort` с компаратором на `extern "C" fn`. Эталонное решение.

use std::cmp::Ordering;
use std::os::raw::{c_int, c_void};

extern "C" {
    fn qsort(
        base: *mut c_void,
        nmemb: usize,
        size: usize,
        compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int,
    );
}

/// Компаратор для `i32`, вызывается из C.
///
/// # Safety
/// `a` и `b` ведут на валидные `i32` (гарантирует qsort).
unsafe extern "C" fn cmp_i32(a: *const c_void, b: *const c_void) -> c_int {
    let a = *(a as *const i32);
    let b = *(b as *const i32);
    match a.cmp(&b) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Отсортировать срез `i32` по возрастанию через libc `qsort`.
pub fn sort_i32(data: &mut [i32]) {
    if data.is_empty() {
        return;
    }
    // SAFETY: base/nmemb/size описывают ровно этот срез; компаратор корректен для i32.
    unsafe {
        qsort(
            data.as_mut_ptr() as *mut c_void,
            data.len(),
            std::mem::size_of::<i32>(),
            cmp_i32,
        );
    }
}
