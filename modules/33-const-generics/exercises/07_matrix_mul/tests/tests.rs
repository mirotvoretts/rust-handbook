use ex_33_07_matrix_mul::Matrix;

#[test]
fn multiply_shapes() {
    let a: Matrix<2, 3> = Matrix::from_rows([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
    let b: Matrix<3, 2> = Matrix::from_rows([[7.0, 8.0], [9.0, 10.0], [11.0, 12.0]]);
    let c: Matrix<2, 2> = a * b;
    assert_eq!(c.get(0, 0), 58.0);
    assert_eq!(c.get(0, 1), 64.0);
    assert_eq!(c.get(1, 0), 139.0);
    assert_eq!(c.get(1, 1), 154.0);
}

#[test]
fn identity_is_neutral() {
    let a: Matrix<2, 2> = Matrix::from_rows([[1.0, 2.0], [3.0, 4.0]]);
    let id: Matrix<2, 2> = Matrix::identity();
    let r = a.clone() * id;
    assert_eq!(r, a);
}

// Не компилируется - размерности не согласованы:
// let a: Matrix<2, 3> = Matrix::zeros();
// let b: Matrix<2, 2> = Matrix::zeros();
// let _ = a * b;
