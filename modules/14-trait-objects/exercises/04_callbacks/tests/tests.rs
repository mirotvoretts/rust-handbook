use ex_14_04_callbacks::Pipeline;

#[test]
fn empty_pipeline_is_identity() {
    assert_eq!(Pipeline::new().run(42), 42);
}

#[test]
fn steps_run_in_order() {
    let mut p = Pipeline::new();
    p.add_step(Box::new(|x| x * 2));
    p.add_step(Box::new(|x| x + 1));
    assert_eq!(p.run(10), 21); // (10*2)+1, не (10+1)*2
}

#[test]
fn offsets_capture_environment() {
    let mut p = Pipeline::new();
    for k in [1, 10, 100] {
        p.add_offset(k);
    }
    assert_eq!(p.run(0), 111);
}
