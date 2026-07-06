use sol_07_12_reverse_halves::reverse_halves;

#[test]
fn even_length() {
    let mut a = [1, 2, 3, 4];
    reverse_halves(&mut a);
    assert_eq!(a, [2, 1, 4, 3]);
}

#[test]
fn odd_length() {
    let mut a = [1, 2, 3, 4, 5];
    reverse_halves(&mut a);
    assert_eq!(a, [2, 1, 5, 4, 3]);
}

#[test]
fn edge_cases() {
    let mut a = [1];
    reverse_halves(&mut a);
    assert_eq!(a, [1]);

    let mut b: [i32; 0] = [];
    reverse_halves(&mut b);
    assert_eq!(b, []);

    let mut c = [1, 2];
    reverse_halves(&mut c);
    assert_eq!(c, [1, 2]);
}
