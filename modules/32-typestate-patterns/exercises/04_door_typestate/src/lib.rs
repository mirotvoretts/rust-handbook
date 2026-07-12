//! 04 (1x) - type-state двери.
//!
//! Закодируй состояние двери в её типе `Door<State>`. Состояния - ZST-метки
//! `Open`, `Closed`, `Locked` (даны ниже). Методы перехода забирают `self` по
//! значению, так что старое состояние после перехода использовать нельзя.
//!
//! Реализуй сам (impl-блоки не даны):
//! - `Door::<Closed>::new() -> Door<Closed>`;
//! - на `Door<Closed>`: `open(self) -> Door<Open>` и `lock(self) -> Door<Locked>`;
//! - на `Door<Open>`: `close(self) -> Door<Closed>` и
//!   `walk_through(&self) -> &'static str` (возвращает `"прошли"`);
//! - на `Door<Locked>`: `unlock(self) -> Door<Closed>`.
//!
//! Поле `_state: PhantomData<State>` инициализируется значением `PhantomData`.
//! Пока impl-блоки не написаны, крейт не компилируется - это нормальное состояние
//! нерешённого упражнения.
//!
//! Про type-state: <https://cliffle.com/blog/rust-typestate/>.
use std::marker::PhantomData;

pub struct Open;
pub struct Closed;
pub struct Locked;

pub struct Door<State> {
    _state: PhantomData<State>,
}
