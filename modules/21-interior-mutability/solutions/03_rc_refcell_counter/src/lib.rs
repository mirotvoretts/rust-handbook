//! 03 (1x) - Rc<RefCell<T>>: разделяемый счётчик. Эталонное решение.

use std::cell::RefCell;
use std::rc::Rc;

/// Разделяемый счётчик.
pub type Counter = Rc<RefCell<i32>>;

/// Новый счётчик со значением 0.
pub fn new_counter() -> Counter {
    Rc::new(RefCell::new(0))
}

/// Ещё одна ручка на тот же счётчик.
pub fn handle(c: &Counter) -> Counter {
    Rc::clone(c)
}

/// Прибавь 1 к общему значению.
pub fn inc(c: &Counter) {
    *c.borrow_mut() += 1;
}

/// Текущее значение.
pub fn value(c: &Counter) -> i32 {
    *c.borrow()
}
