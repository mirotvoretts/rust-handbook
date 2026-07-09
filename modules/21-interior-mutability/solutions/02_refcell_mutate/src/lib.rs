//! 02 (0x) - RefCell<T>: мутация через &. Эталонное решение.

use std::cell::RefCell;

/// Добавь x в конец вектора.
pub fn push(v: &RefCell<Vec<i32>>, x: i32) {
    v.borrow_mut().push(x);
}

/// Длина вектора.
pub fn len(v: &RefCell<Vec<i32>>) -> usize {
    v.borrow().len()
}

/// Удвой каждый элемент на месте.
pub fn double_all(v: &RefCell<Vec<i32>>) {
    for e in v.borrow_mut().iter_mut() {
        *e *= 2;
    }
}
