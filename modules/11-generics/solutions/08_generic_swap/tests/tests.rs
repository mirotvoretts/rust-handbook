use sol_11_08_generic_swap::{exchange, reverse_in_place, swap_ends};

#[test]
fn swap_ends_basic() {
    let mut xs = [1, 2, 3, 4];
    swap_ends(&mut xs);
    assert_eq!(xs, [4, 2, 3, 1]);
}

#[test]
fn swap_ends_short_slices() {
    let mut one = [7];
    swap_ends(&mut one);
    assert_eq!(one, [7]);
    let mut empty: [i32; 0] = [];
    swap_ends(&mut empty);
}

#[test]
fn exchange_returns_old_no_clone_needed() {
    // String не Copy: exchange обязан работать через владение, не через копию
    let mut s = String::from("old");
    let old = exchange(&mut s, String::from("new"));
    assert_eq!(old, "old");
    assert_eq!(s, "new");
}

#[test]
fn reverse_works() {
    let mut xs = [1, 2, 3, 4, 5];
    reverse_in_place(&mut xs);
    assert_eq!(xs, [5, 4, 3, 2, 1]);
    let mut ys: [String; 2] = [String::from("a"), String::from("b")];
    reverse_in_place(&mut ys);
    assert_eq!(ys, [String::from("b"), String::from("a")]);
}
