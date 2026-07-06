use sol_12_03_impl_for_foreign::Weight;

#[test]
fn i32_weight_is_abs() {
    assert_eq!(5i32.weight(), 5);
    assert_eq!((-7i32).weight(), 7);
    assert_eq!(i32::MIN.weight(), 2147483648); // ловушка: -MIN не влезает в i32
}

#[test]
fn strings_weigh_bytes() {
    assert_eq!(String::from("abc").weight(), 3);
    assert_eq!("Ы".weight(), 2); // кириллица - 2 байта в UTF-8
}

#[test]
fn vec_sums_weights() {
    assert_eq!(vec![1, -2, 3].weight(), 6);
    assert_eq!(Vec::<i32>::new().weight(), 0);
}
