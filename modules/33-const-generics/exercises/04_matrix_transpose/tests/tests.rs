use ex_33_04_matrix_transpose::Matrix;

#[test]
fn transpose_shape_and_values() {
    let m: Matrix<2, 3> = Matrix::from_rows([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
    let t: Matrix<3, 2> = m.transpose();
    assert_eq!(t.get(0, 0), 1.0);
    assert_eq!(t.get(1, 0), 2.0);
    assert_eq!(t.get(2, 1), 6.0);
}

#[test]
fn get_set() {
    let mut m: Matrix<2, 2> = Matrix::zeros();
    m.set(1, 1, 9.0);
    assert_eq!(m.get(1, 1), 9.0);
    assert_eq!(m.get(0, 0), 0.0);
}
