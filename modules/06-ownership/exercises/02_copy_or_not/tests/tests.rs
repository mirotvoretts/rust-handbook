use ex_06_02_copy_or_not::{clone_pair, duplicate, Label, Point};

#[test]
fn point_is_copy() {
    let p = Point { x: 1, y: 2 };
    let (a, b) = duplicate(p);
    assert_eq!((a.x, a.y), (1, 2));
    assert_eq!((b.x, b.y), (1, 2));
}

#[test]
fn label_clones() {
    let l = Label(String::from("tag"));
    let (a, b) = clone_pair(&l);
    assert_eq!(a.0, "tag");
    assert_eq!(b.0, "tag");
    // исходная метка всё ещё доступна - clone_pair брал её по ссылке
    assert_eq!(l.0, "tag");
}
