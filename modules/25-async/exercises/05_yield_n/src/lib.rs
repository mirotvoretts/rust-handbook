//! 05 (2x) - Future с явными приостановками и самопробуждением.
//!
//! Реализуй Future `YieldN`, который возвращает `Poll::Pending` ровно `times` раз, а
//! затем становится готовым. Правило пробуждения обязательно: возвращая `Pending`,
//! Future должен сам запросить повторный опрос, вызвав `wake` у `Waker` из `Context`
//! (иначе executor уснёт навсегда - его некому будить). Результатом `Ready` служит
//! общее число вызовов `poll`, потребовавшихся до готовности (то есть `times + 1`):
//! так тест убеждается, что путь `Pending` действительно пройден.
//!
//! block_on(yield_n(0)) == 1   (готов с первого опроса, ни одного Pending)
//! block_on(yield_n(3)) == 4   (3 раза Pending, 4-й опрос - Ready)
//!
//! Поля `YieldN` определи сам. Тип не самоссылочный (`Unpin`) - до `&mut Self` из
//! `Pin<&mut Self>` добирайся через `Pin::get_mut`.
//!
//! Конструкции за пределами теории:
//! - Waker и запрос повторного опроса: Waker::wake_by_ref
//!   https://doc.rust-lang.org/std/task/struct.Waker.html#method.wake_by_ref
//! - смысл Pending и самопробуждения - раздел 4 README и глава async book
//!   "Wakeups": https://rust-lang.github.io/async-book/

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Future, приостанавливающийся (`Pending`) заданное число раз перед готовностью.
pub struct YieldN {
    // определи поля сам
}

/// Построй `YieldN`, который отдаст `Pending` ровно `times` раз.
pub fn yield_n(times: u32) -> YieldN {
    let _ = times;
    todo!()
}

impl Future for YieldN {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let _ = cx;
        todo!()
    }
}
