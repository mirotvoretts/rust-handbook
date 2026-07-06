use ex_18_03_fn_bounds::{apply_twice, eval, for_each};

#[test]
fn apply_twice_composes() {
    assert_eq!(apply_twice(|n| n + 3, 1), 7);
    assert_eq!(apply_twice(|n| n * 2, 5), 20);
}

#[test]
fn for_each_allows_mutating_closure() {
    let mut sum = 0;
    for_each(&[1, 2, 3, 4], |x| sum += x);
    assert_eq!(sum, 10);

    let mut seen = Vec::new();
    for_each(&[10, 20], |x| seen.push(x));
    assert_eq!(seen, vec![10, 20]);
}

#[test]
fn eval_runs_once_and_consumes() {
    let s = String::from("owned");
    let r = eval(move || s);
    assert_eq!(r, "owned");

    assert_eq!(eval(|| 42), 42);
}
