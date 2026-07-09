//! 07 (3x) - наблюдатели через Weak. Эталонное решение.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// Наблюдатель - разделяемая изменяемая ячейка.
pub type Observer = Rc<RefCell<i32>>;

/// Источник событий.
#[derive(Default)]
pub struct Subject {
    observers: Vec<Weak<RefCell<i32>>>,
}

impl Subject {
    /// Пустой источник.
    pub fn new() -> Self {
        Self::default()
    }

    /// Подписать наблюдателя слабой ссылкой.
    pub fn subscribe(&mut self, o: &Observer) {
        self.observers.push(Rc::downgrade(o));
    }

    /// Уведомить всех живых, вернув их число.
    pub fn notify(&mut self, delta: i32) -> usize {
        let mut count = 0;
        for w in &self.observers {
            if let Some(rc) = w.upgrade() {
                *rc.borrow_mut() += delta;
                count += 1;
            }
        }
        count
    }
}
