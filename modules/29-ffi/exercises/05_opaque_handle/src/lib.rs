//! 05 (2x) - непрозрачный указатель (opaque handle) (раздел 5 README).
//!
//! Иногда C должен хранить объект Rust между вызовами, не заглядывая внутрь. Идиома -
//! отдать наружу `*mut T` как безымянный токен, а работу вести через экспортированные
//! функции. Передача владения кучей: `Box::into_raw` отдаёт указатель и снимает с `Box`
//! ответственность за освобождение; `Box::from_raw` возвращает владение обратно в Rust.
//!
//! Тип `Counter` задан. Реализуй C-подобное API (атрибуты уже проставлены):
//! - `counter_new()` -> `*mut Counter` - выделить `Counter { value: 0 }` на куче и отдать
//!   сырой указатель (`Box::into_raw(Box::new(...))`).
//! - `counter_add(c, n)` - прибавить `n` к значению по указателю. Указатель в ссылку
//!   удобно превратить через `c.as_mut()` (вернёт `None` на null).
//! - `counter_value(c)` -> `i64` - вернуть текущее значение (0 на null).
//! - `counter_free(c)` - вернуть владение и уронить объект (`Box::from_raw`), но только
//!   если указатель не null.
//!
//! Контракт: на каждый `counter_new` ровно один `counter_free`. Ни разу - утечка; дважды -
//! double-free и UB.
//!
//! `Box::into_raw`/`from_raw`:
//! <https://doc.rust-lang.org/std/boxed/struct.Box.html#method.into_raw>.

pub struct Counter {
    value: i64,
}

/// Выделить счётчик на куче, отдать владение наружу.
#[no_mangle]
pub extern "C" fn counter_new() -> *mut Counter {
    todo!()
}

/// Прибавить `n` к значению счётчика.
///
/// # Safety
/// `c` получен из `counter_new` и ещё не освобождён (или null).
#[no_mangle]
pub unsafe extern "C" fn counter_add(c: *mut Counter, n: i64) {
    let _ = (c, n);
    todo!()
}

/// Текущее значение счётчика.
///
/// # Safety
/// `c` получен из `counter_new` и ещё не освобождён (или null).
#[no_mangle]
pub unsafe extern "C" fn counter_value(c: *const Counter) -> i64 {
    let _ = c;
    todo!()
}

/// Освободить счётчик; после вызова указатель невалиден.
///
/// # Safety
/// `c` получен из `counter_new`; вызывается ровно один раз (или null).
#[no_mangle]
pub unsafe extern "C" fn counter_free(c: *mut Counter) {
    let _ = c;
    todo!()
}
