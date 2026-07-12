//! 06 (2x) - гашение паники на границе FFI. Эталонное решение.

use std::os::raw::c_int;
use std::panic::{catch_unwind, AssertUnwindSafe};

/// Внутренняя Rust-логика: паникует на делении на ноль.
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }
    a / b
}

/// Деление с гашением паники. Возврат: 0 - успех (`*out` записан), 1 - была паника.
///
/// # Safety
/// `out` - валидный указатель на `c_int` (или null).
#[no_mangle]
pub unsafe extern "C" fn checked_div(a: c_int, b: c_int, out: *mut c_int) -> c_int {
    match catch_unwind(AssertUnwindSafe(|| divide(a, b))) {
        Ok(v) => {
            if !out.is_null() {
                *out = v;
            }
            0
        }
        Err(_) => 1,
    }
}
