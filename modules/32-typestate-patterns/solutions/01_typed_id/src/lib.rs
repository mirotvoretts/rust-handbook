//! 01 (0x) - типизированный id на PhantomData. Эталонное решение.
use std::marker::PhantomData;

pub struct User;
pub struct Post;

pub struct Id<T> {
    value: u64,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(value: u64) -> Self {
        Id { value, _marker: PhantomData }
    }
    pub fn value(&self) -> u64 {
        self.value
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Id<T> {}
impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl<T> std::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id({})", self.value)
    }
}

pub fn user_name(id: Id<User>) -> String {
    format!("user #{}", id.value())
}
