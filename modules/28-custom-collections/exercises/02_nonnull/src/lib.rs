//! 02 (0x) - `NonNull<T>` и доступ через сырой указатель (раздел 2 README).
//!
//! `std::ptr::NonNull<T>` - это `*mut T` с гарантией "не null" и с ковариантностью по
//! `T` (в отличие от `*mut T`, который инвариантен). Именно `NonNull` хранят внутри
//! своих буферов `Box`, `Vec`, `Rc`. В этом упражнении данные уже лежат в памяти
//! (их даёт вызывающий срез), выделять ничего не нужно - только строить `NonNull` и
//! читать/писать через него.
//!
//! Реализуй:
//! - `first_ptr::<T>(slice)` -> `Option<NonNull<T>>` (безопасная): `NonNull` на первый
//!   элемент среза; для пустого среза - `None`. Возьми `slice.as_mut_ptr()` и заверни в
//!   `NonNull::new` (он вернёт `None` на нулевом указателе).
//! - `unsafe read_at::<T: Copy>(p)` -> `T`: прочитать значение по указателю
//!   (`p.as_ptr().read()`).
//! - `unsafe write_at::<T>(p, val)`: записать значение (`p.as_ptr().write(val)`).
//! - `unsafe offset::<T>(p, i)` -> `NonNull<T>`: указатель на элемент со сдвигом `i`
//!   (в единицах элементов `T`, как `p.as_ptr().offset(i)`), снова как `NonNull`.
//!
//! Инвариант вызывающего (почему функции `unsafe`): указатель должен вести на живой,
//! инициализированный, корректно выровненный элемент `T`, а `offset` - оставаться в
//! пределах одного выделения. Проверить это компилятор не может - за это отвечает тот,
//! кто вызывает.
//!
//! Документация: <https://doc.rust-lang.org/std/ptr/struct.NonNull.html>,
//! методы указателя `read`/`write`/`offset`:
//! <https://doc.rust-lang.org/std/primitive.pointer.html>.

use std::ptr::NonNull;

/// `NonNull` на первый элемент; `None`, если срез пуст.
pub fn first_ptr<T>(slice: &mut [T]) -> Option<NonNull<T>> {
    let _ = slice;
    todo!()
}

/// # Safety
/// `p` ведёт на живой, инициализированный, выровненный `T`.
pub unsafe fn read_at<T: Copy>(p: NonNull<T>) -> T {
    let _ = p;
    todo!()
}

/// # Safety
/// `p` ведёт на живой, выровненный `T`, куда разрешена запись.
pub unsafe fn write_at<T>(p: NonNull<T>, val: T) {
    let _ = (p, val);
    todo!()
}

/// # Safety
/// `p.offset(i)` остаётся в пределах того же выделения.
pub unsafe fn offset<T>(p: NonNull<T>, i: isize) -> NonNull<T> {
    let _ = (p, i);
    todo!()
}
