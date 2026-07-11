use ex_25_06_mini_block_on::block_on;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[test]
fn ready_immediately() {
    assert_eq!(block_on(async { 1 + 2 }), 3);
}

// Future, который один раз возвращает Pending (с самопробуждением), затем Ready.
struct YieldOnce(bool);

impl Future for YieldOnce {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<i32> {
        let this = self.get_mut();
        if this.0 {
            Poll::Ready(42)
        } else {
            this.0 = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[test]
fn drives_pending_future() {
    assert_eq!(block_on(YieldOnce(false)), 42);
}

#[test]
fn drives_await_chain_with_suspensions() {
    let out = block_on(async {
        let a = YieldOnce(false).await;
        let b = YieldOnce(false).await;
        a + b
    });
    assert_eq!(out, 84);
}
