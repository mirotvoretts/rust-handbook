use sol_08_10_refs_longer_than::refs_longer_than;

#[test]
fn filters() {
    assert_eq!(
        refs_longer_than(&["a", "bbb", "cc", "dddd"], 2),
        vec!["bbb", "dddd"]
    );
    assert_eq!(refs_longer_than(&["a", "bb", "c"], 5), Vec::<&str>::new());
    assert_eq!(
        refs_longer_than(&["one", "two", "three"], 0),
        vec!["one", "two", "three"]
    );
}
