use ex_04_08_derive_debug_eq::{debug_string, same, Point};

#[test]
fn equality() {
    let a = Point { x: 1, y: 2 };
    let b = Point { x: 1, y: 2 };
    let c = Point { x: 9, y: 2 };
    assert!(same(&a, &b));
    assert!(!same(&a, &c));
    // прямое сравнение работает благодаря #[derive(PartialEq, Debug)]
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn formatting() {
    let p = Point { x: 1, y: 2 };
    assert_eq!(debug_string(&p), "Point { x: 1, y: 2 }");
}
