//! 07 (3x) - умножение матриц, размерности в типе. Эталонное решение.
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<const R: usize, const C: usize> {
    data: [[f64; C]; R],
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn from_rows(data: [[f64; C]; R]) -> Self {
        Matrix { data }
    }
    pub fn zeros() -> Self {
        Matrix { data: [[0.0; C]; R] }
    }
    pub fn get(&self, r: usize, c: usize) -> f64 {
        self.data[r][c]
    }
}

impl<const N: usize> Matrix<N, N> {
    pub fn identity() -> Self {
        let mut m = Matrix::<N, N>::zeros();
        let mut i = 0;
        while i < N {
            m.data[i][i] = 1.0;
            i += 1;
        }
        m
    }
}

impl<const M: usize, const N: usize, const P: usize> Mul<Matrix<N, P>> for Matrix<M, N> {
    type Output = Matrix<M, P>;
    fn mul(self, rhs: Matrix<N, P>) -> Matrix<M, P> {
        let mut out = Matrix::<M, P>::zeros();
        for i in 0..M {
            for j in 0..P {
                let mut acc = 0.0;
                for k in 0..N {
                    acc += self.data[i][k] * rhs.data[k][j];
                }
                out.data[i][j] = acc;
            }
        }
        out
    }
}
