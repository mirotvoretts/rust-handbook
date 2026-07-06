use ex_10_01_define_module::arithmetic;

#[test]
fn module_functions() {
    assert_eq!(arithmetic::add(2, 3), 5);
    assert_eq!(arithmetic::mul(4, 5), 20);
}
