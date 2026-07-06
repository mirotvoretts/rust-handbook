//! 09 (2x) - Индекс-кортеж для сетки.
//!
//! Сетка width x height хранится плоским Vec (row-major: ячейка (r, c) лежит по
//! смещению r * width + c). Index возвращает ССЫЛКУ - сахар xs[i] сам добавит
//! разыменование. IndexMut делает grid[(r, c)] = v присваиваемым.

pub struct Grid {
    width: usize,
    cells: Vec<f64>,
}

impl Grid {
    /// Сетка width x height, заполненная нулями.
    pub fn new(width: usize, height: usize) -> Self {
        todo!("vec![0.0; width * height]")
    }
}

impl std::ops::Index<(usize, usize)> for Grid {
    type Output = f64;

    fn index(&self, (r, c): (usize, usize)) -> &f64 {
        todo!("&self.cells[...]")
    }
}

impl std::ops::IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut f64 {
        todo!()
    }
}
