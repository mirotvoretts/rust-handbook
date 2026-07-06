use ex_05_12_expr_eval::{eval, Expr};

fn num(n: i64) -> Box<Expr> {
    Box::new(Expr::Num(n))
}

#[test]
fn single_number() {
    assert_eq!(eval(&Expr::Num(42)), 42);
}

#[test]
fn negation() {
    assert_eq!(eval(&Expr::Neg(num(5))), -5);
}

#[test]
fn nested() {
    // (2 + 3) * 4 == 20
    let expr = Expr::Mul(Box::new(Expr::Add(num(2), num(3))), num(4));
    assert_eq!(eval(&expr), 20);

    // -(1 + 2) * (3 + 4) == -21
    let left = Box::new(Expr::Neg(Box::new(Expr::Add(num(1), num(2)))));
    let right = Box::new(Expr::Add(num(3), num(4)));
    assert_eq!(eval(&Expr::Mul(left, right)), -21);
}
