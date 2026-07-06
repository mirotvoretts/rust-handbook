use sol_17_10_word_index::WordIndex;

#[test]
fn positions_case_insensitive() {
    let idx = WordIndex::build("The cat saw the dog");
    assert_eq!(idx.find("the"), &[0, 3]);
    assert_eq!(idx.find("THE"), &[0, 3]);
    assert_eq!(idx.find("cat"), &[1]);
    assert_eq!(idx.find("missing"), &[] as &[usize]);
}

#[test]
fn top_n_with_tiebreak() {
    let idx = WordIndex::build("b a b a c b");
    // b:3, a:2, c:1
    assert_eq!(
        idx.top_n(2),
        vec![(String::from("b"), 3), (String::from("a"), 2)]
    );
    // равные частоты -> лексикографический порядок
    let idx = WordIndex::build("z y z y");
    assert_eq!(
        idx.top_n(5),
        vec![(String::from("y"), 2), (String::from("z"), 2)]
    );
}

#[test]
fn empty_text() {
    let idx = WordIndex::build("");
    assert_eq!(idx.top_n(3), vec![]);
}
