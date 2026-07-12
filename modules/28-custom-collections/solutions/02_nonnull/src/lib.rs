//! 02 (0x) - `NonNull<T>` и доступ через сырой указатель. Эталонное решение.

use std::ptr::NonNull;

/// `NonNull` на первый элемент; `None`, если срез пуст.
pub fn first_ptr<T>(slice: &mut [T]) -> Option<NonNull<T>> {
    if slice.is_empty() {
        None
    } else {
        NonNull::new(slice.as_mut_ptr())
    }
}

/// # Safety
/// `p` ведёт на живой, инициализированный, выровненный `T`.
pub unsafe fn read_at<T: Copy>(p: NonNull<T>) -> T {
    p.as_ptr().read()
}

/// # Safety
/// `p` ведёт на живой, выровненный `T`, куда разрешена запись.
pub unsafe fn write_at<T>(p: NonNull<T>, val: T) {
    p.as_ptr().write(val);
}

/// # Safety
/// `p.offset(i)` остаётся в пределах того же выделения.
pub unsafe fn offset<T>(p: NonNull<T>, i: isize) -> NonNull<T> {
    NonNull::new_unchecked(p.as_ptr().offset(i))
}
