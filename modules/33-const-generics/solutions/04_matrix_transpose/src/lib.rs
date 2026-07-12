//! 04 (1x) - транспонирование матрицы с размерами в типе. Эталонное решение.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<const R: usize, const C: usize> {
    data: [[f64; C]; R],
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn zeros() -> Self {
        Matrix { data: [[0.0; C]; R] }
    }
    pub fn from_rows(data: [[f64; C]; R]) -> Self {
        Matrix { data }
    }
    pub fn get(&self, r: usize, c: usize) -> f64 {
        self.data[r][c]
    }
    pub fn set(&mut self, r: usize, c: usize, value: f64) {
        self.data[r][c] = value;
    }
    pub fn transpose(&self) -> Matrix<C, R> {
        let mut out = Matrix::<C, R>::zeros();
        for i in 0..R {
            for j in 0..C {
                out.data[j][i] = self.data[i][j];
            }
        }
        out
    }
}
