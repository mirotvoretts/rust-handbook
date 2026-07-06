use sol_03_05_swap_via_mut::{add_assign, swap};

#[test]
fn swapping() {
    let mut x = 1;
    let mut y = 2;
    swap(&mut x, &mut y);
    assert_eq!(x, 2);
    assert_eq!(y, 1);
}

#[test]
fn adding() {
    let mut x = 10;
    add_assign(&mut x, 5);
    assert_eq!(x, 15);
    add_assign(&mut x, -20);
    assert_eq!(x, -5);
}
