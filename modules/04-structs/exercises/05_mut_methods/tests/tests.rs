use ex_04_05_mut_methods::Counter;

#[test]
fn counting() {
    let mut c = Counter::new();
    assert_eq!(c.value(), 0);
    c.increment();
    c.increment();
    assert_eq!(c.value(), 2);
    c.add(10);
    assert_eq!(c.value(), 12);
    c.reset();
    assert_eq!(c.value(), 0);
}
