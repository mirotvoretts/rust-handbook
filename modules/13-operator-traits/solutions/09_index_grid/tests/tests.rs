use sol_13_09_index_grid::Grid;

#[test]
fn starts_zeroed() {
    let g = Grid::new(3, 2);
    assert_eq!(g[(0, 0)], 0.0);
    assert_eq!(g[(1, 2)], 0.0);
}

#[test]
fn write_then_read() {
    let mut g = Grid::new(4, 3);
    g[(2, 1)] = 7.5;
    g[(0, 3)] = -1.0;
    assert_eq!(g[(2, 1)], 7.5);
    assert_eq!(g[(0, 3)], -1.0);
    assert_eq!(g[(2, 2)], 0.0); // соседняя ячейка не задета
}

#[test]
fn compound_assign_through_index_mut() {
    let mut g = Grid::new(2, 2);
    g[(1, 1)] += 2.0; // IndexMut + AddAssign у f64
    g[(1, 1)] *= 3.0;
    assert_eq!(g[(1, 1)], 6.0);
}

#[test]
#[should_panic]
fn out_of_bounds_panics() {
    let g = Grid::new(2, 2);
    let _ = g[(0, 5)]; // смещение 0*2+5=5 — за пределами cells (len 4)
}
