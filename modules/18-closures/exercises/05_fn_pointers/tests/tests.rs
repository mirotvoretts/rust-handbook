use ex_18_05_fn_pointers::{as_fn_pointer, dispatch, run_pipeline};

#[test]
fn dispatch_known_ops() {
    assert_eq!(dispatch("add").unwrap()(2, 3), 5);
    assert_eq!(dispatch("sub").unwrap()(2, 3), -1);
    assert_eq!(dispatch("mul").unwrap()(2, 3), 6);
}

#[test]
fn dispatch_unknown_is_none() {
    assert!(dispatch("div").is_none());
    assert!(dispatch("").is_none());
}

#[test]
fn pipeline_applies_left_to_right() {
    // замыкания без захватов приводятся к fn(i64) -> i64
    let ops: &[fn(i64) -> i64] = &[|x| x + 1, |x| x * 10];
    assert_eq!(run_pipeline(3, ops), 40); // (3 + 1) * 10
    assert_eq!(run_pipeline(5, &[]), 5);
}

#[test]
fn captureless_closure_as_fn() {
    let p = as_fn_pointer();
    assert_eq!(p(21), 42);
    // однородность: fn-указатели складываются в Vec
    let table: Vec<fn(i64) -> i64> = vec![p, |x| x + 100];
    assert_eq!(table[1](1), 101);
}
