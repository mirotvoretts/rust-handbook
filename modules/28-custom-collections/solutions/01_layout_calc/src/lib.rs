//! 01 (0x) - вычисление `Layout` для массива. Эталонное решение.

use std::alloc::Layout;

/// Размер в байтах массива `[T; n]`; `None`, если размер переполняет предел.
pub fn array_bytes<T>(n: usize) -> Option<usize> {
    Layout::array::<T>(n).ok().map(|l| l.size())
}

/// `(size, align)` массива `[T; n]`; `None` при переполнении.
pub fn layout_of<T>(n: usize) -> Option<(usize, usize)> {
    Layout::array::<T>(n).ok().map(|l| (l.size(), l.align()))
}

/// Расстояние в байтах между соседними элементами массива `T`.
pub fn elem_stride<T>() -> usize {
    std::mem::size_of::<T>()
}
