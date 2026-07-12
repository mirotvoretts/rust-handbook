//! 07 (3x) - вызов libc `qsort` с компаратором на `extern "C" fn` (раздел 8 README).
//!
//! `qsort` из libc сортирует произвольный массив, а сравнение элементов делегирует
//! callback - функции с C-ABI. Прототип:
//!
//! ```c
//! void qsort(void *base, size_t nmemb, size_t size,
//!            int (*compar)(const void *, const void *));
//! ```
//!
//! compar получает два элемента как `*const c_void` и возвращает отрицательное / ноль /
//! положительное число (меньше / равно / больше).
//!
//! Сделай:
//! - объяви `extern "C"` блок с `qsort` (тип compar -
//!   `unsafe extern "C" fn(*const c_void, *const c_void) -> c_int`).
//! - напиши компаратор `unsafe extern "C" fn` для `i32`: привести оба указателя к
//!   `*const i32`, прочитать значения и вернуть `c_int` по правилу выше. Готовый порядок
//!   даёт `Ord::cmp`, а `Ordering` кладётся в `-1/0/1` через сопоставление.
//! - `sort_i32(data)` - вызвать `qsort` над срезом: `base` = `data.as_mut_ptr() as *mut
//!   c_void`, `nmemb` = длина, `size` = `size_of::<i32>()`, compar - твой компаратор.
//!   Пустой срез обработай отдельно (в `qsort` пустой массив передавать не нужно).
//!
//! Указатель на обычную (не захватывающую) функцию раскладывается в C-указатель;
//! замыкание с захватом - нет. Справка:
//! <https://doc.rust-lang.org/reference/types/function-pointer.html>.

use std::os::raw::{c_int, c_void};

// TODO: объяви extern "C" блок с qsort и напиши компаратор для i32.

/// Отсортировать срез `i32` по возрастанию через libc `qsort`.
pub fn sort_i32(data: &mut [i32]) {
    let _ = data;
    todo!()
}
