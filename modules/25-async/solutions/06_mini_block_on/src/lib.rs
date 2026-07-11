//! 06 (2x) - свой блокирующий executor. Эталонное решение.

use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

/// Waker для одной задачи: "разбудить" = снять текущий поток с park.
struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.unpark();
    }
}

pub fn block_on<F: Future>(future: F) -> F::Output {
    // закрепляем Future на стеке: poll требует Pin<&mut F>.
    let mut future = std::pin::pin!(future);

    // Waker, будящий поток, в котором крутится этот block_on.
    let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
    let mut cx = Context::from_waker(&waker);

    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(value) => return value,
            // park снимает поток с процессора; unpark из wake() вернёт его сюда.
            // Если wake уже был вызван до park, токен unpark сохранён - park не
            // заснёт вхолостую.
            Poll::Pending => thread::park(),
        }
    }
}
