use ex_09_02_reborrow::{bump, bump_n, bump_twice};

#[test]
fn single() {
    let mut x = 5;
    bump(&mut x);
    assert_eq!(x, 6);
}

#[test]
fn twice() {
    let mut x = 5;
    bump_twice(&mut x);
    assert_eq!(x, 7);
}

#[test]
fn n_times() {
    let mut x = 0;
    bump_n(&mut x, 10);
    assert_eq!(x, 10);

    let mut y = 3;
    bump_n(&mut y, 0);
    assert_eq!(y, 3);
}
