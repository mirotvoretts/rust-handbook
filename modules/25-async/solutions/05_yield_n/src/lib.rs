//! 05 (2x) - Future с явными приостановками и самопробуждением. Эталонное решение.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct YieldN {
    remaining: u32,
    polls: u32,
}

pub fn yield_n(times: u32) -> YieldN {
    YieldN {
        remaining: times,
        polls: 0,
    }
}

impl Future for YieldN {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut(); // YieldN: Unpin
        this.polls += 1;

        if this.remaining == 0 {
            Poll::Ready(this.polls)
        } else {
            this.remaining -= 1;
            // сами просим executor опросить нас ещё раз, иначе он уснёт навсегда.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
