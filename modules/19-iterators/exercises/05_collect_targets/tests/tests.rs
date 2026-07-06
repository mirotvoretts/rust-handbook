use ex_19_05_collect_targets::{parse_all, shout, word_lengths};

#[test]
fn collects_into_string() {
    assert_eq!(shout("hello"), "HELLO");
    assert_eq!(shout(""), "");
}

#[test]
fn collects_into_hashmap() {
    let m = word_lengths(&["cat", "hi", "cat"]);
    assert_eq!(m.get("cat"), Some(&3));
    assert_eq!(m.get("hi"), Some(&2));
    assert_eq!(m.len(), 2); // "cat" схлопнулся в один ключ
}

#[test]
fn collect_result_ok() {
    assert_eq!(parse_all(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
    assert_eq!(parse_all(&[]), Ok(vec![]));
}

#[test]
fn collect_result_short_circuits() {
    let r = parse_all(&["1", "oops", "3"]);
    assert!(r.is_err());
}
