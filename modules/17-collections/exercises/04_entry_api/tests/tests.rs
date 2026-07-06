use ex_17_04_entry_api::{group_by_first_char, word_count};

#[test]
fn counts_words() {
    let c = word_count("a b a c a b");
    assert_eq!(c["a"], 3);
    assert_eq!(c["b"], 2);
    assert_eq!(c["c"], 1);
    assert_eq!(c.len(), 3);
}

#[test]
fn empty_text() {
    assert!(word_count("   ").is_empty());
}

#[test]
fn groups_preserve_order() {
    let g = group_by_first_char(&["apple", "banana", "avocado", ""]);
    assert_eq!(g[&'a'], vec!["apple", "avocado"]);
    assert_eq!(g[&'b'], vec!["banana"]);
    assert_eq!(g.len(), 2); // пустое слово пропущено
}
