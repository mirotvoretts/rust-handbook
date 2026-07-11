//! 06 (2x) - свой блокирующий executor.
//!
//! Реализуй `block_on(future)` - минимальный executor на примитивах стандартной
//! библиотеки, без сторонних крейтов. Он опрашивает единственный Future в цикле,
//! пока тот не вернёт `Ready`, а на `Pending` усыпляет текущий поток, чтобы не жечь
//! процессор. Разбудить поток обязан `Waker`, который executor передаёт в `poll`:
//! когда Future вызовет `wake`, спящий поток должен проснуться и снова опросить
//! Future.
//!
//! Схема (раздел 4 README): poll -> Ready? вернуть; Pending? park; wake -> unpark.
//!
//! Что понадобится (за пределами теории):
//! - закрепить Future на стеке для опроса: макрос std::pin::pin!
//!   https://doc.rust-lang.org/std/pin/macro.pin.html
//! - построить Waker из своего типа: трейт std::task::Wake (реализуется для Arc<T>),
//!   затем Waker::from(Arc::new(...))
//!   https://doc.rust-lang.org/std/task/trait.Wake.html
//! - Context::from_waker: https://doc.rust-lang.org/std/task/struct.Context.html
//! - усыпить/разбудить поток: std::thread::park и Thread::unpark
//!   https://doc.rust-lang.org/std/thread/fn.park.html
//! - пример устройства executor'а - глава async book "Build an Executor":
//!   https://rust-lang.github.io/async-book/

use std::future::Future;

/// Крути `future` до готовности, усыпляя поток между опросами, и верни результат.
pub fn block_on<F: Future>(future: F) -> F::Output {
    let _ = future;
    todo!()
}
