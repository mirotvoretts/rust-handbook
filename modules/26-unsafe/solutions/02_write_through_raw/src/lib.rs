//! 02 (0x) - запись через `*mut`. Эталонное решение.

pub fn set_via_raw(target: &mut i64, value: i64) {
    let p: *mut i64 = target;
    unsafe { *p = value } // p ведёт на живой i64, эксклюзивно взятый из &mut
}
