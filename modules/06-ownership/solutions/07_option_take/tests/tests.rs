use sol_06_07_option_take::{drain_vec, replace_with, take_out};

#[test]
fn take_out_leaves_none() {
    let mut slot = Some(String::from("payload"));
    let got = take_out(&mut slot);
    assert_eq!(got, Some(String::from("payload")));
    assert_eq!(slot, None);

    // повторный take из None даёт None
    assert_eq!(take_out(&mut slot), None);
}

#[test]
fn replace_returns_old() {
    let mut slot = String::from("old");
    let old = replace_with(&mut slot, String::from("new"));
    assert_eq!(old, "old");
    assert_eq!(slot, "new");
}

#[test]
fn drain_leaves_empty() {
    let mut v = vec![1, 2, 3];
    let taken = drain_vec(&mut v);
    assert_eq!(taken, vec![1, 2, 3]);
    assert_eq!(v, Vec::<i32>::new());
}
