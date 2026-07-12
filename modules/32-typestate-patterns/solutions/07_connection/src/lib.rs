//! 07 (3x) - протокол соединения на type-state + sealed. Эталонное решение.
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

impl Connection<Disconnected> {
    pub fn new() -> Self {
        Connection { addr: String::new(), user: None, _state: PhantomData }
    }
    pub fn connect(self, addr: &str) -> Connection<Connected> {
        Connection { addr: addr.to_string(), user: None, _state: PhantomData }
    }
}

impl Default for Connection<Disconnected> {
    fn default() -> Self {
        Self::new()
    }
}

impl Connection<Connected> {
    pub fn authenticate(self, user: &str) -> Connection<Authenticated> {
        Connection { addr: self.addr, user: Some(user.to_string()), _state: PhantomData }
    }
    pub fn disconnect(self) -> Connection<Disconnected> {
        Connection { addr: String::new(), user: None, _state: PhantomData }
    }
}

impl Connection<Authenticated> {
    pub fn query(&self, sql: &str) -> String {
        format!("{}@{}: {}", self.user.as_ref().unwrap(), self.addr, sql)
    }
    pub fn disconnect(self) -> Connection<Disconnected> {
        Connection { addr: String::new(), user: None, _state: PhantomData }
    }
}

impl<S: State> Connection<S> {
    pub fn address(&self) -> &str {
        &self.addr
    }
}
