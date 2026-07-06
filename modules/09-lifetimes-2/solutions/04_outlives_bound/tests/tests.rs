use sol_09_04_outlives_bound::as_shorter;

#[test]
fn shortens() {
    assert_eq!(as_shorter("hello"), "hello");

    let owned = String::from("world");
    let len;
    {
        let r = as_shorter(&owned);
        len = r.len();
    }
    assert_eq!(len, 5);
}
