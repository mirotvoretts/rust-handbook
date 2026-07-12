//! 05 (2x) - непрозрачный указатель (opaque handle). Эталонное решение.

pub struct Counter {
    value: i64,
}

/// Выделить счётчик на куче, отдать владение наружу.
#[no_mangle]
pub extern "C" fn counter_new() -> *mut Counter {
    Box::into_raw(Box::new(Counter { value: 0 }))
}

/// Прибавить `n` к значению счётчика.
///
/// # Safety
/// `c` получен из `counter_new` и ещё не освобождён (или null).
#[no_mangle]
pub unsafe extern "C" fn counter_add(c: *mut Counter, n: i64) {
    if let Some(cnt) = c.as_mut() {
        cnt.value += n;
    }
}

/// Текущее значение счётчика.
///
/// # Safety
/// `c` получен из `counter_new` и ещё не освобождён (или null).
#[no_mangle]
pub unsafe extern "C" fn counter_value(c: *const Counter) -> i64 {
    match c.as_ref() {
        Some(cnt) => cnt.value,
        None => 0,
    }
}

/// Освободить счётчик; после вызова указатель невалиден.
///
/// # Safety
/// `c` получен из `counter_new`; вызывается ровно один раз (или null).
#[no_mangle]
pub unsafe extern "C" fn counter_free(c: *mut Counter) {
    if !c.is_null() {
        drop(Box::from_raw(c));
    }
}
