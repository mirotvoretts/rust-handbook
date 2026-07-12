//! 08 (3x) - callback с протаскиванием состояния через `void*` (раздел 8 README).
//!
//! Многие C-API принимают callback (`extern "C" fn`) плюс нетипизированный `void*`
//! "userdata", который прокидывают в callback нетронутым. Так протаскивают состояние:
//! замыкание с захватом в C передать нельзя (у него нет стабильного адреса), поэтому
//! контекст кладут в userdata, а внутри callback приводят его обратно к своему типу.
//!
//! Функция `run_events` (дана, изображает C-библиотеку) на каждое событие зовёт `cb`,
//! прокидывая `userdata`. Реализуй `count_matching(events, target)`: сколько раз в
//! `events` встречается `target`, но подсчёт вести *через* `run_events`, а не циклом
//! напрямую. Для этого:
//! - опиши тип-контекст (например, с полями `target` и счётчиком) и создай его;
//! - напиши callback `extern "C" fn(c_int, *mut c_void)`: привести `userdata` обратно к
//!   `*mut Контекст`, разыменовать и обновить счётчик, если событие равно target;
//! - вызови `run_events`, передав указатель на контекст как `*mut c_void`
//!   (`&mut ctx as *mut _ as *mut c_void`), затем верни накопленный счётчик.
//!
//! Приведение `*mut c_void` -> `*mut T` и разыменование - `unsafe`. Справка по указателям
//! на функции: <https://doc.rust-lang.org/reference/types/function-pointer.html>.

use std::os::raw::{c_int, c_void};

/// C-подобная библиотека (менять не нужно): на каждое событие зовёт `cb` с `userdata`.
pub fn run_events(
    events: &[c_int],
    cb: extern "C" fn(c_int, *mut c_void),
    userdata: *mut c_void,
) {
    for &e in events {
        cb(e, userdata);
    }
}

/// Сколько раз `target` встречается в `events`, подсчёт через `run_events`.
pub fn count_matching(events: &[c_int], target: c_int) -> usize {
    let _ = (events, target);
    todo!()
}
