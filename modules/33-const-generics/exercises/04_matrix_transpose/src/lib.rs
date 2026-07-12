//! 04 (1x) - транспонирование матрицы с размерами в типе.
//!
//! Реализуй матрицу `Matrix<const R: usize, const C: usize>` поверх `[[f64; C]; R]`.
//! Требуемое API:
//! - `zeros()` - нулевая матрица;
//! - `from_rows(data: [[f64; C]; R])` - из массива строк;
//! - `get(r, c)`, `set(r, c, value)`;
//! - `transpose(&self) -> Matrix<C, R>` - транспонированная матрица (размеры
//!   меняются местами на уровне типа).
//!
//! Подсказки:
//! - нулевую матрицу собери литералом `[[0.0; C]; R]`;
//! - для транспонирования создай `Matrix::<C, R>::zeros()` и заполни `out[j][i]`
//!   значением `self[i][j]`.
//!
//! Заглушки помечены `todo!()` - крейт компилируется, тесты падают, пока не решено.
//! Про const-generics:
//! <https://doc.rust-lang.org/reference/items/generics.html#const-generics>.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<const R: usize, const C: usize> {
    data: [[f64; C]; R],
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn zeros() -> Self {
        todo!()
    }
    pub fn from_rows(data: [[f64; C]; R]) -> Self {
        let _ = data;
        todo!()
    }
    pub fn get(&self, r: usize, c: usize) -> f64 {
        let _ = (r, c);
        todo!()
    }
    pub fn set(&mut self, r: usize, c: usize, value: f64) {
        let _ = (r, c, value);
        todo!()
    }
    pub fn transpose(&self) -> Matrix<C, R> {
        todo!()
    }
}
