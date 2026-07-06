use ex_20_05_rc_sharing::{shared_tail, strong, sum};

#[test]
fn heads_share_one_tail() {
    let (a, b, tail) = shared_tail();
    assert_eq!(sum(&a), 6); // 1 + 2 + 3
    assert_eq!(sum(&b), 14); // 9 + 2 + 3
    // на хвост ссылаются: сам tail + голова a + голова b = 3
    assert_eq!(strong(&tail), 3);
}

#[test]
fn clone_bumps_count() {
    let (_a, _b, tail) = shared_tail();
    let before = strong(&tail);
    let extra = std::rc::Rc::clone(&tail);
    assert_eq!(strong(&tail), before + 1);
    drop(extra);
    assert_eq!(strong(&tail), before); // счётчик вернулся
}

#[test]
fn dropping_a_head_lowers_count() {
    let (a, _b, tail) = shared_tail();
    assert_eq!(strong(&tail), 3);
    drop(a);
    assert_eq!(strong(&tail), 2); // одна голова ушла, хвост жив
}
