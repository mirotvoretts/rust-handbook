use sol_13_08_assign_ops::Resources;

#[test]
fn add_assign_accumulates() {
    let mut r = Resources(10);
    r += 5;
    r += 1;
    assert_eq!(r, Resources(16));
}

#[test]
fn sub_assign_saturates() {
    let mut r = Resources(10);
    r -= 3;
    assert_eq!(r, Resources(7));
    r -= 100; // не паника и не переполнение: насыщение в 0
    assert_eq!(r, Resources(0));
}
