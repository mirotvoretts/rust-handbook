use sol_06_11_swap_replace::{rotate_left3, swap_two};

#[test]
fn swap_swaps() {
    let mut a = String::from("left");
    let mut b = String::from("right");
    swap_two(&mut a, &mut b);
    assert_eq!(a, "right");
    assert_eq!(b, "left");
}

#[test]
fn rotate_moves_c_into_a() {
    let mut a = String::from("A");
    let mut b = String::from("B");
    let mut c = String::from("C");
    rotate_left3(&mut a, &mut b, &mut c);
    assert_eq!(a, "C");
    assert_eq!(b, "A");
    assert_eq!(c, "B");
}

#[test]
fn rotate_works_on_ints() {
    let (mut a, mut b, mut c) = (1, 2, 3);
    rotate_left3(&mut a, &mut b, &mut c);
    assert_eq!((a, b, c), (3, 1, 2));
}
