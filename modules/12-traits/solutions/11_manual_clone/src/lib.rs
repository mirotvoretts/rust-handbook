//! 11 (3x) — Ловушка derive: лишняя граница. Эталонное решение.

use std::marker::PhantomData;

/// Типизированный идентификатор: у TypedId<User> и TypedId<Order> разные типы.
pub struct TypedId<T> {
    pub id: u64,
    pub _marker: PhantomData<T>,
}

impl<T> TypedId<T> {
    pub fn new(id: u64) -> Self {
        TypedId { id, _marker: PhantomData }
    }
}

impl<T> Clone for TypedId<T> {
    fn clone(&self) -> Self {
        TypedId { id: self.id, _marker: PhantomData }
    }
}

impl<T> Copy for TypedId<T> {}
