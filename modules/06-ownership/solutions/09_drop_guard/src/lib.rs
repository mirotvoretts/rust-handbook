//! 09 (2x) — RAII-гвард с побочным эффектом на `Drop`. Эталонное решение.

use std::cell::RefCell;
use std::rc::Rc;

pub struct Transaction {
    name: String,
    log: Rc<RefCell<Vec<String>>>,
}

impl Transaction {
    pub fn begin(name: &str, log: Rc<RefCell<Vec<String>>>) -> Transaction {
        log.borrow_mut().push(format!("begin {name}"));
        Transaction {
            name: name.to_string(),
            log,
        }
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        self.log.borrow_mut().push(format!("end {}", self.name));
    }
}
