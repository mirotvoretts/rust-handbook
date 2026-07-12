//! 07 (3x) - умножение матриц, размерности в типе.
//!
//! Реализуй умножение матриц с проверкой согласованности размеров на уровне типа.
//! `Matrix<const R: usize, const C: usize>` поверх `[[f64; C]; R]`. Требуемое API:
//! - `from_rows(data)`, `zeros()`, `get(r, c)`;
//! - `identity()` - только для квадратных `Matrix<N, N>`;
//! - `impl Mul<Matrix<N, P>> for Matrix<M, N>` с `Output = Matrix<M, P>`.
//!
//! Умножение несогласованных матриц (когда число столбцов левой не равно числу
//! строк правой) должно быть ошибкой компиляции - это обеспечивают const-параметры
//! в сигнатуре `Mul`.
//!
//! Подсказки:
//! - `identity` вынесен в отдельный `impl<const N: usize> Matrix<N, N>`;
//! - произведение: `out[i][j] = sum_k self[i][k] * rhs[k][j]`.
//!
//! Заглушки помечены `todo!()` - крейт компилируется, тесты падают, пока не решено.
//! Про трейт Mul: <https://doc.rust-lang.org/std/ops/trait.Mul.html>.
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<const R: usize, const C: usize> {
    data: [[f64; C]; R],
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn from_rows(data: [[f64; C]; R]) -> Self {
        let _ = data;
        todo!()
    }
    pub fn zeros() -> Self {
        todo!()
    }
    pub fn get(&self, r: usize, c: usize) -> f64 {
        let _ = (r, c);
        todo!()
    }
}

impl<const N: usize> Matrix<N, N> {
    pub fn identity() -> Self {
        todo!()
    }
}

impl<const M: usize, const N: usize, const P: usize> Mul<Matrix<N, P>> for Matrix<M, N> {
    type Output = Matrix<M, P>;
    fn mul(self, rhs: Matrix<N, P>) -> Matrix<M, P> {
        let _ = rhs;
        todo!()
    }
}
