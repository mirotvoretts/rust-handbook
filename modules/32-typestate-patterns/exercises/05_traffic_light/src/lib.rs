//! 05 (2x) - светофор на type-state.
//!
//! Смоделируй светофор как конечный автомат, где текущий свет закодирован в типе
//! `Light<State>`. Состояния `Red`, `Green`, `Yellow` и трейт `TrafficState` с
//! ассоциированной константой `NAME` уже даны (это контракт задачи).
//!
//! Реализуй сам (impl-блоки для `Light` не даны):
//! - общий метод `color(&self) -> &'static str` для любого `Light<S: TrafficState>`,
//!   возвращающий `S::NAME`;
//! - `Light::<Red>::new() -> Light<Red>`;
//! - на каждом состоянии метод `next(self)`, переводящий по циклу
//!   Red -> Green -> Yellow -> Red.
//!
//! Поле `_state: PhantomData<State>` инициализируется значением `PhantomData`.
//! Пока impl-блоки не написаны, крейт не компилируется - это нормальное состояние
//! нерешённого упражнения.
//!
//! Про type-state: <https://cliffle.com/blog/rust-typestate/>.
use std::marker::PhantomData;

pub trait TrafficState {
    const NAME: &'static str;
}

pub struct Red;
pub struct Green;
pub struct Yellow;

impl TrafficState for Red {
    const NAME: &'static str = "red";
}
impl TrafficState for Green {
    const NAME: &'static str = "green";
}
impl TrafficState for Yellow {
    const NAME: &'static str = "yellow";
}

pub struct Light<State> {
    _state: PhantomData<State>,
}
