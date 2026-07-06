//! 10 (2x) — Почему move НЕ приводит к double-free. Эталонное решение.

use std::cell::RefCell;
use std::rc::Rc;

pub struct Resource {
    id: u32,
    freed: Rc<RefCell<Vec<u32>>>,
}

impl Resource {
    pub fn new(id: u32, freed: Rc<RefCell<Vec<u32>>>) -> Resource {
        Resource { id, freed }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        self.freed.borrow_mut().push(self.id);
    }
}

/// Владение просто движется дальше — источник moved-from, его Drop не вызывается.
pub fn relay(r: Resource) -> Resource {
    r
}
