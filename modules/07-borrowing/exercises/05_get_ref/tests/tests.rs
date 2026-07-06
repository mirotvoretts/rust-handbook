use ex_07_05_get_ref::{find, get};

#[test]
fn getting() {
    let xs = [10, 20, 30];
    assert_eq!(get(&xs, 1), Some(&20));
    assert_eq!(get(&xs, 3), None);
    assert_eq!(get(&[], 0), None);
}

#[test]
fn finding() {
    let xs = [4, 5, 6, 5];
    assert_eq!(find(&xs, 5), Some(&5));
    assert_eq!(find(&xs, 9), None);
}
