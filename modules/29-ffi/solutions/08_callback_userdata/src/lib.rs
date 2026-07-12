//! 08 (3x) - callback с протаскиванием состояния через `void*`. Эталонное решение.

use std::os::raw::{c_int, c_void};

/// C-подобная библиотека: на каждое событие зовёт `cb` с `userdata`.
pub fn run_events(
    events: &[c_int],
    cb: extern "C" fn(c_int, *mut c_void),
    userdata: *mut c_void,
) {
    for &e in events {
        cb(e, userdata);
    }
}

/// Контекст, протаскиваемый через userdata.
struct Ctx {
    target: c_int,
    count: usize,
}

/// Callback: приводит userdata обратно к `Ctx` и считает совпадения.
extern "C" fn on_event(event: c_int, userdata: *mut c_void) {
    // SAFETY: userdata - это &mut Ctx, переданный из count_matching; он жив на весь вызов
    // run_events, и callback вызывается однопоточно, без пересечения заимствований.
    let ctx = unsafe { &mut *(userdata as *mut Ctx) };
    if event == ctx.target {
        ctx.count += 1;
    }
}

/// Сколько раз `target` встречается в `events`, подсчёт через `run_events`.
pub fn count_matching(events: &[c_int], target: c_int) -> usize {
    let mut ctx = Ctx { target, count: 0 };
    run_events(events, on_event, &mut ctx as *mut Ctx as *mut c_void);
    ctx.count
}
