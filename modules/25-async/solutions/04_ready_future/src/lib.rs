//! 04 (1x) - реализация трейта Future вручную. Эталонное решение.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Ready(pub Option<i64>);

impl Future for Ready {
    type Output = i64;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Ready: Unpin -> get_mut безопасен. take() забирает значение по владению;
        // Waker не нужен вовсе - мы никогда не возвращаем Pending.
        let value = self
            .get_mut()
            .0
            .take()
            .expect("Ready опрошен повторно после готовности");
        Poll::Ready(value)
    }
}
