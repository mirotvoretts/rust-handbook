//! 04 (1x) - type-state двери. Эталонное решение.
use std::marker::PhantomData;

pub struct Open;
pub struct Closed;
pub struct Locked;

pub struct Door<State> {
    _state: PhantomData<State>,
}

impl Door<Closed> {
    pub fn new() -> Self {
        Door { _state: PhantomData }
    }
    pub fn open(self) -> Door<Open> {
        Door { _state: PhantomData }
    }
    pub fn lock(self) -> Door<Locked> {
        Door { _state: PhantomData }
    }
}

impl Default for Door<Closed> {
    fn default() -> Self {
        Self::new()
    }
}

impl Door<Open> {
    pub fn close(self) -> Door<Closed> {
        Door { _state: PhantomData }
    }
    pub fn walk_through(&self) -> &'static str {
        "прошли"
    }
}

impl Door<Locked> {
    pub fn unlock(self) -> Door<Closed> {
        Door { _state: PhantomData }
    }
}
