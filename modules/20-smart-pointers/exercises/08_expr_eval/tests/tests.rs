use ex_20_08_expr_eval::{add, eval, mul, neg, node_count, num, sub};

#[test]
fn evaluates_leaf() {
    assert_eq!(eval(&num(42)), 42);
    assert_eq!(eval(&num(-5)), -5);
}

#[test]
fn evaluates_binary_ops() {
    assert_eq!(eval(&add(num(2), num(3))), 5);
    assert_eq!(eval(&sub(num(10), num(4))), 6);
    assert_eq!(eval(&mul(num(6), num(7))), 42);
}

#[test]
fn evaluates_unary_neg() {
    assert_eq!(eval(&neg(num(9))), -9);
    assert_eq!(eval(&neg(neg(num(3)))), 3);
}

#[test]
fn evaluates_nested() {
    // (2 + 3) * -(4 - 1) = 5 * -3 = -15
    let e = mul(add(num(2), num(3)), neg(sub(num(4), num(1))));
    assert_eq!(eval(&e), -15);
}

#[test]
fn counts_nodes() {
    // add(num, num): 3 узла
    assert_eq!(node_count(&add(num(1), num(2))), 3);
    // (2+3)*-(4-1): mul, add, 2, 3, neg, sub, 4, 1 = 8
    let e = mul(add(num(2), num(3)), neg(sub(num(4), num(1))));
    assert_eq!(node_count(&e), 8);
    assert_eq!(node_count(&num(0)), 1);
}
