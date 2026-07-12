//! 03 (1x) - кольцевой буфер на const generic.
//!
//! Реализуй кольцевой буфер фиксированной ёмкости `RingBuffer<T, const N: usize>`
//! поверх массива `[Option<T>; N]`. Требуемое API:
//! - `new()` - пустой буфер;
//! - `capacity()` - ёмкость (равна `N`);
//! - `len()`, `is_empty()`;
//! - `push(value)` - добавить элемент; при заполнении перезаписывает самый старый;
//! - `pop()` - извлечь самый старый элемент (FIFO), либо `None`, если пусто.
//!
//! Подсказки:
//! - массив инициализируй через `std::array::from_fn(|_| None)`;
//! - храни индекс головы и длину; позиция хвоста - `(head + len) % N`, перенос по
//!   модулю `N`.
//!
//! Заглушки помечены `todo!()` - крейт компилируется, тесты падают, пока не решено.
//! Про array::from_fn:
//! <https://doc.rust-lang.org/std/array/fn.from_fn.html>.
pub struct RingBuffer<T, const N: usize> {
    data: [Option<T>; N],
    head: usize,
    len: usize,
}

impl<T, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        todo!()
    }
    pub fn capacity(&self) -> usize {
        todo!()
    }
    pub fn len(&self) -> usize {
        todo!()
    }
    pub fn is_empty(&self) -> bool {
        todo!()
    }
    pub fn push(&mut self, value: T) {
        let _ = value;
        todo!()
    }
    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }
}
