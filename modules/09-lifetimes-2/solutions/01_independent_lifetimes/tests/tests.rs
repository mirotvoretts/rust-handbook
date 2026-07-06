use sol_09_01_independent_lifetimes::keep_first;

#[test]
fn returns_first() {
    assert_eq!(keep_first("hello", "world"), "hello");
}

#[test]
fn second_may_be_short_lived() {
    let long = String::from("keep me");
    let out;
    {
        let short = String::from("temporary");
        out = keep_first(&long, &short);
    }
    assert_eq!(out, "keep me");
}
