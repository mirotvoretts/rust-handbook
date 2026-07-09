//! 04 (1x) - try_borrow / try_borrow_mut без паники. Эталонное решение.

use std::cell::RefCell;

/// Прочитай значение, если нет активного мутабельного заёма, иначе None.
pub fn read_if_free(c: &RefCell<i32>) -> Option<i32> {
    c.try_borrow().ok().map(|r| *r)
}

/// Прибавь delta, вернув Ok(новое); если значение занято заёмом - Err(()).
#[allow(clippy::result_unit_err)]
pub fn add_checked(c: &RefCell<i32>, delta: i32) -> Result<i32, ()> {
    match c.try_borrow_mut() {
        Ok(mut v) => {
            *v += delta;
            Ok(*v)
        }
        Err(_) => Err(()),
    }
}
