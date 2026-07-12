//! 04 (1x) - экспорт функций Rust под C-ABI. Эталонное решение.

use std::os::raw::c_int;

/// Сумма двух `c_int`, wrapping при переполнении.
#[no_mangle]
pub extern "C" fn add_i32(a: c_int, b: c_int) -> c_int {
    a.wrapping_add(b)
}

/// Сумма массива `i32`, пришедшего как указатель + длина.
///
/// # Safety
/// `ptr` ведёт на `len` инициализированных `i32` в одном выделении (либо null при len 0).
#[no_mangle]
pub unsafe extern "C" fn sum_array(ptr: *const i32, len: usize) -> i64 {
    if ptr.is_null() {
        return 0;
    }
    let slice = std::slice::from_raw_parts(ptr, len);
    slice.iter().map(|&x| x as i64).sum()
}
