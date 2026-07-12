//! 05 (2x) - светофор на type-state. Эталонное решение.
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

impl<S: TrafficState> Light<S> {
    pub fn color(&self) -> &'static str {
        S::NAME
    }
}

impl Light<Red> {
    pub fn new() -> Self {
        Light { _state: PhantomData }
    }
    pub fn next(self) -> Light<Green> {
        Light { _state: PhantomData }
    }
}

impl Default for Light<Red> {
    fn default() -> Self {
        Self::new()
    }
}

impl Light<Green> {
    pub fn next(self) -> Light<Yellow> {
        Light { _state: PhantomData }
    }
}

impl Light<Yellow> {
    pub fn next(self) -> Light<Red> {
        Light { _state: PhantomData }
    }
}
