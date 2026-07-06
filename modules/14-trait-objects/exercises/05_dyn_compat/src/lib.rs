//! 05 (2x) - Dyn compatibility. ПОЧИНИ СЛОМАННОЕ.
//!
//! Крейт НЕ КОМПИЛИРУЕТСЯ: `error[E0038]` - трейт Buffer нельзя превратить в dyn,
//! потому что метод `fresh_copy` возвращает Self по значению. Почините, ДОБАВИВ к
//! методу ограничение `where Self: Sized` (метод спрячется от trait object, остальное
//! останется работать). Затем реализуйте todo!().

pub trait Buffer {
    fn len(&self) -> usize;

    fn push_byte(&mut self, b: u8);

    /// Свежая пустая копия того же типа. Возвращает Self по значению -
    /// именно это ломает dyn compatibility.
    fn fresh_copy(&self) -> Self;
}

pub struct VecBuffer {
    pub bytes: Vec<u8>,
}

impl Buffer for VecBuffer {
    fn len(&self) -> usize {
        todo!()
    }

    fn push_byte(&mut self, b: u8) {
        todo!()
    }

    fn fresh_copy(&self) -> VecBuffer {
        todo!("пустой VecBuffer")
    }
}

/// Заполняет буфер байтами 0..n через dyn-ссылку.
pub fn fill(buf: &mut dyn Buffer, n: u8) {
    todo!("цикл push_byte")
}
