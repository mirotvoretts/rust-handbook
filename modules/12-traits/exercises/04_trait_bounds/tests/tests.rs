use ex_12_04_trait_bounds::{describe_all, describe_pair, Circle, Square};

#[test]
fn all_same_type() {
    let shapes = [Circle { radius: 1.0 }, Circle { radius: 2.0 }];
    assert_eq!(describe_all(&shapes), vec!["круг r=1", "круг r=2"]);
}

#[test]
fn pair_of_different_types() {
    let c = Circle { radius: 1.5 };
    let s = Square { side: 3.0 };
    assert_eq!(describe_pair(&c, &s), "круг r=1.5 и квадрат a=3");
}

#[test]
fn empty_slice() {
    let none: [Square; 0] = [];
    assert_eq!(describe_all(&none), Vec::<String>::new());
}
