//! 04 (1x) - свой `Box<T>` (разделы 3-4 README).
//!
//! `MyBox<T>` владеет одним значением `T` на куче. Нужно связать воедино четыре вещи:
//! выделение и запись значения (`new`), прозрачный доступ (`Deref`/`DerefMut`),
//! освобождение с запуском деструктора значения (`Drop`).
//!
//! Тип уже объявлен: `MyBox<T> { ptr: NonNull<T> }`. Реализуй:
//! - `new(value)` -> `Self`: выделить `Layout::new::<T>()`, при нулевом указателе -
//!   `handle_alloc_error`, записать `value` через `.write(value)`, сохранить `NonNull`.
//!   Для простоты считаем, что `T` ненулевого размера (ZST разбираются в упр. 08);
//!   можно оставить `assert_ne!(size_of::<T>(), 0, ...)` в начале.
//! - `Deref`/`DerefMut` с `Target = T`: вернуть `&T` / `&mut T` через `ptr.as_ref()` /
//!   `ptr.as_mut()`. После этого `*b`, вызовы методов `T` и т.п. работают как у `Box`.
//! - `Drop`: сначала уронить *значение* по адресу (`std::ptr::drop_in_place(ptr)`),
//!   затем освободить память (`dealloc` с тем же layout). Порядок важен: деструктор
//!   значения должен отработать до освобождения его памяти.
//!
//! Почему `drop_in_place`, а не просто `dealloc`: `dealloc` возвращает память
//! аллокатору, но НЕ запускает `Drop` у `T`. Если `T` владеет ресурсами (например,
//! `String`), без `drop_in_place` они утекут.
//!
//! `Deref`/`DerefMut`: <https://doc.rust-lang.org/std/ops/trait.Deref.html>.
//! `drop_in_place`: <https://doc.rust-lang.org/std/ptr/fn.drop_in_place.html>.

use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

pub struct MyBox<T> {
    ptr: NonNull<T>,
}

impl<T> MyBox<T> {
    pub fn new(value: T) -> Self {
        let _ = value;
        todo!()
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        todo!()
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        todo!()
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        todo!()
    }
}
