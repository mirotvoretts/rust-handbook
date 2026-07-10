use ex_24_04_pipeline::pipeline;

#[test]
fn doubles() {
    assert_eq!(pipeline(vec![1, 2, 3], |x| x * 2), vec![2, 4, 6]);
}

#[test]
fn preserves_order() {
    assert_eq!(pipeline(vec![5, 1, 3], |x| x + 100), vec![105, 101, 103]);
}

#[test]
fn empty() {
    assert_eq!(pipeline(vec![], |x| x), Vec::<i64>::new());
}

#[test]
fn captures_environment() {
    let factor = 10;
    assert_eq!(pipeline(vec![1, 2, 3], move |x| x * factor), vec![10, 20, 30]);
}
