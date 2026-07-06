use sol_15_02_impl_trait_return::{compose, make_adder, make_scaler};

#[test]
fn adder_captures_k() {
    let add5 = make_adder(5);
    assert_eq!(add5(1), 6);
    assert_eq!(add5(-5), 0);
}

#[test]
fn scaler() {
    let x3 = make_scaler(3);
    assert_eq!(x3(7), 21);
}

#[test]
fn composition_order() {
    let add_then_scale = compose(make_adder(1), make_scaler(10));
    assert_eq!(add_then_scale(4), 50); // (4+1)*10, не 4*10+1
}
