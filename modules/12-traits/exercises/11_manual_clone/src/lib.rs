//! 11 (3x) — Ловушка derive: лишняя граница. ПОЧИНИ СЛОМАННОЕ.
//!
//! Тесты этого крейта НЕ КОМПИЛИРУЮТСЯ, пока вы не допишете impl Copy (внизу файла).
//!
//! `TypedId<T>` — числовой идентификатор, «помеченный» типом T для типобезопасности
//! (id пользователя нельзя перепутать с id заказа). T НЕ ХРАНИТСЯ: PhantomData<T> —
//! маркер нулевого размера, который лишь «притворяется» полем T (подробнее в M32).
//!
//! `#[derive(Clone, Copy)]` сгенерировал бы `impl<T: Clone> ...` — и TypedId<NotClone>
//! перестал бы копироваться, хотя копируется тут только u64! Напишите impl руками,
//! БЕЗ границы на T. Тесты проверяют клонирование именно с не-Clone параметром.

use std::marker::PhantomData;

/// Типизированный идентификатор: у TypedId<User> и TypedId<Order> разные типы.
pub struct TypedId<T> {
    pub id: u64,
    pub _marker: PhantomData<T>,
}

impl<T> TypedId<T> {
    pub fn new(id: u64) -> Self {
        todo!("PhantomData — просто значение PhantomData")
    }
}

// todo!(): impl<T> Clone for TypedId<T> — БЕЗ T: Clone
impl<T> Clone for TypedId<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

// todo!(): impl<T> Copy for TypedId<T> — у Copy нет методов
// (раскомментируйте и допишите)
// impl<T> Copy for TypedId<T> {}
