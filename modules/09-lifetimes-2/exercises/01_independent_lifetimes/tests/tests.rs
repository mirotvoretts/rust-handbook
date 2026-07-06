use ex_09_01_independent_lifetimes::keep_first;

#[test]
fn returns_first() {
    assert_eq!(keep_first("hello", "world"), "hello");
}

#[test]
fn second_may_be_short_lived() {
    let long = String::from("keep me");
    let out;
    {
        // second живёт только внутри этого блока...
        let short = String::from("temporary");
        out = keep_first(&long, &short);
    }
    // ...но out заимствует только long, поэтому всё ещё валиден
    assert_eq!(out, "keep me");
}
