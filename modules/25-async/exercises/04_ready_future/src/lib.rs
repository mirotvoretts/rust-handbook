//! 04 (1x) - реализация трейта Future вручную.
//!
//! `Ready` - простейший Future: он готов сразу и при первом же опросе отдаёт
//! хранимое внутри значение, ни разу не возвращая `Pending`. Реализуй для него трейт
//! `Future` так, чтобы `.await` над `Ready(Some(v))` давал `v`. Значение лежит в
//! `Option`, чтобы его можно было забрать (`take`) из `poll` по владению; повторный
//! опрос после готовности - нарушение контракта Future, на нём допустимо паниковать.
//!
//! Сигнатура `poll` задана трейтом (см. раздел 3). Тип `Ready` не самоссылочный,
//! поэтому он `Unpin`, и до `&mut Self` из `Pin<&mut Self>` можно добраться безопасно
//! через `Pin::get_mut`.
//!
//! Конструкции за пределами теории:
//! - трейт Future и его метод poll:
//!   https://doc.rust-lang.org/std/future/trait.Future.html
//! - Poll (Ready/Pending): https://doc.rust-lang.org/std/task/enum.Poll.html
//! - Pin и Pin::get_mut: https://doc.rust-lang.org/std/pin/struct.Pin.html#method.get_mut

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Future, готовый немедленно: отдаёт значение из `Option` при первом опросе.
pub struct Ready(pub Option<i64>);

impl Future for Ready {
    type Output = i64;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let _ = cx;
        todo!()
    }
}
