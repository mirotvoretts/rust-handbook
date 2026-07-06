//! 13 (3x) - Дерево владения на `Box` и рекурсивный `Drop`. Эталонное решение.

use std::cell::RefCell;
use std::rc::Rc;

pub struct Tree {
    id: u32,
    // поля никогда не читаются - их роль только в том, чтобы владеть поддеревьями,
    // и Drop сработает на них рекурсивно в правильном порядке
    #[allow(dead_code)]
    left: Option<Box<Tree>>,
    #[allow(dead_code)]
    right: Option<Box<Tree>>,
    log: Rc<RefCell<Vec<u32>>>,
}

impl Tree {
    pub fn leaf(id: u32, log: Rc<RefCell<Vec<u32>>>) -> Tree {
        Tree {
            id,
            left: None,
            right: None,
            log,
        }
    }

    pub fn node(
        id: u32,
        left: Option<Tree>,
        right: Option<Tree>,
        log: Rc<RefCell<Vec<u32>>>,
    ) -> Tree {
        Tree {
            id,
            left: left.map(Box::new),
            right: right.map(Box::new),
            log,
        }
    }
}

impl Drop for Tree {
    fn drop(&mut self) {
        self.log.borrow_mut().push(self.id);
    }
}
