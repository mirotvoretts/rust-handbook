//! 03 (1x) - строки через границу FFI: `CString`/`CStr` (раздел 3 README).
//!
//! Строка C - это указатель на байты, завершённые нулевым байтом. `CString` строит такую
//! владеющую строку из `&str`, `CStr` заимствует её по сырому указателю. В этом упражнении
//! длину строки считает не Rust, а функция C `size_t strlen(const char*)` из libc.
//!
//! Сделай:
//! - объяви `extern "C"` блок с `strlen` (аргумент `*const c_char`, результат `usize`).
//! - `to_c(s)` -> `Result<CString, NulError>` - построить `CString` из `&str`
//!   (`CString::new` вернёт `Err`, если внутри строки есть нулевой байт).
//! - `c_strlen(c)` -> `usize` - длину `CStr` через вызов libc `strlen` (передать
//!   `c.as_ptr()`).
//! - `from_c_ptr(ptr)` -> `String` (`unsafe`) - прочитать C-строку по указателю обратно
//!   в `String` (`CStr::from_ptr(ptr)` + `to_string_lossy`).
//!
//! Осторожно с временем жизни: указатель от `as_ptr()` валиден, только пока жив сам
//! `CString`.
//!
//! Документация: <https://doc.rust-lang.org/std/ffi/struct.CString.html>,
//! <https://doc.rust-lang.org/std/ffi/struct.CStr.html>.

use std::ffi::{CStr, CString, NulError};
use std::os::raw::c_char;

// TODO: объяви здесь extern "C" блок с функцией strlen.

/// Построить `CString` из строки.
pub fn to_c(s: &str) -> Result<CString, NulError> {
    let _ = s;
    todo!()
}

/// Длина C-строки через libc `strlen`.
pub fn c_strlen(c: &CStr) -> usize {
    let _ = c;
    todo!()
}

/// Прочитать C-строку по указателю в `String`.
///
/// # Safety
/// `ptr` - валидный, нуль-терминированный указатель на живую C-строку.
pub unsafe fn from_c_ptr(ptr: *const c_char) -> String {
    let _ = ptr;
    todo!()
}
