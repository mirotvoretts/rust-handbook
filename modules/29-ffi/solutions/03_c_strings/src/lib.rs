//! 03 (1x) - строки через границу FFI: `CString`/`CStr`. Эталонное решение.

use std::ffi::{CStr, CString, NulError};
use std::os::raw::c_char;

extern "C" {
    // size_t strlen(const char*); из libc.
    fn strlen(s: *const c_char) -> usize;
}

/// Построить `CString` из строки.
pub fn to_c(s: &str) -> Result<CString, NulError> {
    CString::new(s)
}

/// Длина C-строки через libc `strlen`.
pub fn c_strlen(c: &CStr) -> usize {
    // SAFETY: c.as_ptr() - валидный нуль-терминированный указатель, живой на время вызова.
    unsafe { strlen(c.as_ptr()) }
}

/// Прочитать C-строку по указателю в `String`.
///
/// # Safety
/// `ptr` - валидный, нуль-терминированный указатель на живую C-строку.
pub unsafe fn from_c_ptr(ptr: *const c_char) -> String {
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}
