//! 04 (0x) — `Drop` и порядок уничтожения. Эталонное решение.

use std::cell::RefCell;
use std::rc::Rc;

pub struct Noisy {
    id: u32,
    log: Rc<RefCell<Vec<u32>>>,
}

impl Noisy {
    pub fn new(id: u32, log: Rc<RefCell<Vec<u32>>>) -> Noisy {
        Noisy { id, log }
    }
}

impl Drop for Noisy {
    fn drop(&mut self) {
        self.log.borrow_mut().push(self.id);
    }
}
