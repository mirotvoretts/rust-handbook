use sol_30_05_swap::swap;

#[test]
fn swaps_values() {
    let mut a = 1;
    let mut b = 2;
    swap!(a, b);
    assert_eq!((a, b), (2, 1));
}

#[test]
fn swaps_strings() {
    let mut a = String::from("left");
    let mut b = String::from("right");
    swap!(a, b);
    assert_eq!(a, "right");
    assert_eq!(b, "left");
}

#[test]
fn is_hygienic() {
    let mut a = 10;
    let mut b = 20;
    // Тот же идентификатор, что временная переменная внутри макроса.
    let tmp = 999;
    swap!(a, b);
    assert_eq!((a, b), (20, 10));
    assert_eq!(tmp, 999); // пользовательский tmp не затёрт
}
