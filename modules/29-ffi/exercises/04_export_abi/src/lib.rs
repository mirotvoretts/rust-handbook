//! 04 (1x) - экспорт функций Rust под C-ABI (раздел 4 README).
//!
//! Чтобы функцию Rust мог вызвать C, её помечают `extern "C"` (C-ABI) и `#[no_mangle]`
//! (сохранить имя, без name mangling). Аргументы и результат - только FFI-совместимые типы:
//! числа и сырые указатели, но не `&str`/`String`/`Vec`. Массив из C приходит парой
//! "указатель + длина".
//!
//! Сделай (сигнатуры и атрибуты `#[no_mangle]`/`extern "C"` уже заданы, допиши тела):
//! - `add_i32(a, b)` - сумма, при переполнении не паниковать (`wrapping_add`).
//! - `sum_array(ptr, len)` - сумма `len` элементов `i32`, лежащих по `ptr`, как `i64`.
//!   На нулевой указатель вернуть 0. Срез из указателя собери через
//!   `std::slice::from_raw_parts` (это `unsafe`, потому функция помечена `unsafe`).
//!
//! Справка: <https://doc.rust-lang.org/reference/items/external-blocks.html> и
//! <https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html>.

use std::os::raw::c_int;

/// Сумма двух `c_int`, wrapping при переполнении.
#[no_mangle]
pub extern "C" fn add_i32(a: c_int, b: c_int) -> c_int {
    let _ = (a, b);
    todo!()
}

/// Сумма массива `i32`, пришедшего как указатель + длина.
///
/// # Safety
/// `ptr` ведёт на `len` инициализированных `i32` в одном выделении (либо null при len 0).
#[no_mangle]
pub unsafe extern "C" fn sum_array(ptr: *const i32, len: usize) -> i64 {
    let _ = (ptr, len);
    todo!()
}
