//! 09 (2x) — Индекс-кортеж для сетки. Эталонное решение.

pub struct Grid {
    width: usize,
    cells: Vec<f64>,
}

impl Grid {
    /// Сетка width x height, заполненная нулями.
    pub fn new(width: usize, height: usize) -> Self {
        Grid { width, cells: vec![0.0; width * height] }
    }
}

impl std::ops::Index<(usize, usize)> for Grid {
    type Output = f64;

    fn index(&self, (r, c): (usize, usize)) -> &f64 {
        &self.cells[r * self.width + c]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut f64 {
        &mut self.cells[r * self.width + c]
    }
}
