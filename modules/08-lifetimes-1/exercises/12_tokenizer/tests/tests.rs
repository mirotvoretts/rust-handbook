use ex_08_12_tokenizer::Tokenizer;

#[test]
fn splits_words() {
    let mut t = Tokenizer::new("  hello   world  ");
    assert_eq!(t.next_word(), Some("hello"));
    assert_eq!(t.next_word(), Some("world"));
    assert_eq!(t.next_word(), None);
    assert_eq!(t.next_word(), None);
}

#[test]
fn single_and_empty() {
    let mut t = Tokenizer::new("hi");
    assert_eq!(t.next_word(), Some("hi"));
    assert_eq!(t.next_word(), None);

    let mut e = Tokenizer::new("   ");
    assert_eq!(e.next_word(), None);

    let mut empty = Tokenizer::new("");
    assert_eq!(empty.next_word(), None);
}

#[test]
fn collect_words() {
    // выданные слова живут столько же, сколько исходная строка, поэтому их можно накопить
    let mut t = Tokenizer::new("a bb ccc");
    let mut words: Vec<&str> = Vec::new();
    while let Some(w) = t.next_word() {
        words.push(w);
    }
    assert_eq!(words, vec!["a", "bb", "ccc"]);
}
