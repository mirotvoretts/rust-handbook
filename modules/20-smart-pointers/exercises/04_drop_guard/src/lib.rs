//! 04 (1x) - Drop: RAII-гвард и ранний drop.
//!
//! Drop::drop запускается, когда значение уходит из области видимости. Здесь
//! гвард при уничтожении инкрементит общий счётчик. Счётчик - Rc<AtomicUsize>:
//! Rc даёт разделяемое владение (тема модуля), а AtomicUsize можно менять через
//! общую ссылку (обычный i32 так нельзя - см. следующий модуль про мутацию).

use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// RAII-гвард: держит общий счётчик и увеличивает его на 1 при уничтожении.
pub struct Guard {
    counter: Rc<AtomicUsize>,
}

impl Guard {
    /// Создать гвард поверх общего счётчика.
    pub fn new(counter: Rc<AtomicUsize>) -> Self {
        todo!("Guard { counter }")
    }
}

impl Drop for Guard {
    /// При уничтожении: counter += 1 (fetch_add с Ordering::SeqCst).
    fn drop(&mut self) {
        todo!("self.counter.fetch_add(1, Ordering::SeqCst);")
    }
}

/// Прочитать текущее значение счётчика (сколько гвардов уже уничтожено).
pub fn dropped(counter: &Rc<AtomicUsize>) -> usize {
    todo!("counter.load(Ordering::SeqCst)")
}
