//! 05 (2x) - Dyn compatibility. Эталонное решение.

pub trait Buffer {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn push_byte(&mut self, b: u8);

    /// Свежая пустая копия того же типа. `where Self: Sized` прячет метод от
    /// trait object - и трейт снова dyn-совместим.
    fn fresh_copy(&self) -> Self
    where
        Self: Sized;
}

pub struct VecBuffer {
    pub bytes: Vec<u8>,
}

impl Buffer for VecBuffer {
    fn len(&self) -> usize {
        self.bytes.len()
    }

    fn push_byte(&mut self, b: u8) {
        self.bytes.push(b);
    }

    fn fresh_copy(&self) -> VecBuffer {
        VecBuffer { bytes: Vec::new() }
    }
}

/// Заполняет буфер байтами 0..n через dyn-ссылку.
pub fn fill(buf: &mut dyn Buffer, n: u8) {
    for b in 0..n {
        buf.push_byte(b);
    }
}
