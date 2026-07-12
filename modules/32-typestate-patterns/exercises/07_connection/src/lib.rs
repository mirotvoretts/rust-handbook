//! 07 (3x) - протокол соединения на type-state + sealed.
//!
//! Смоделируй соединение с состояниями `Disconnected`, `Connected`,
//! `Authenticated`, закодированными в типе `Connection<S>`. Множество состояний
//! запечатано sealed-трейтом `State` (обвязка sealing уже дана). Между переходами
//! переносятся данные: адрес и имя пользователя.
//!
//! Реализуй сам (impl-блоки для `Connection` не даны):
//! - `Connection::<Disconnected>::new()`;
//! - на `Disconnected`: `connect(self, addr: &str) -> Connection<Connected>`;
//! - на `Connected`: `authenticate(self, user: &str) -> Connection<Authenticated>`
//!   и `disconnect(self) -> Connection<Disconnected>`;
//! - на `Authenticated`: `query(&self, sql: &str) -> String` в формате
//!   `"<user>@<addr>: <sql>"` и `disconnect(self) -> Connection<Disconnected>`;
//! - общий `address(&self) -> &str` для любого `Connection<S: State>`.
//!
//! Подсказки:
//! - при `connect` сохрани адрес, при `authenticate` перенеси адрес и запиши
//!   пользователя; `disconnect` сбрасывает данные;
//! - поле `_state: PhantomData<S>` инициализируется значением `PhantomData`.
//!
//! Пока impl-блоки не написаны, крейт не компилируется - это нормальное состояние
//! нерешённого упражнения.
//! Про type-state: <https://cliffle.com/blog/rust-typestate/>; про sealed:
//! <https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed>.
use std::marker::PhantomData;

mod sealed {
    pub trait Sealed {}
}

pub trait State: sealed::Sealed {}

pub struct Disconnected;
pub struct Connected;
pub struct Authenticated;

impl sealed::Sealed for Disconnected {}
impl sealed::Sealed for Connected {}
impl sealed::Sealed for Authenticated {}
impl State for Disconnected {}
impl State for Connected {}
impl State for Authenticated {}

pub struct Connection<S: State> {
    addr: String,
    user: Option<String>,
    _state: PhantomData<S>,
}
