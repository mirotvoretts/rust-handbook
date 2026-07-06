use ex_05_03_match_ranges_or::{classify, grade, is_vowel};

#[test]
fn grades() {
    assert_eq!(grade(100), 'A');
    assert_eq!(grade(90), 'A');
    assert_eq!(grade(85), 'B');
    assert_eq!(grade(70), 'C');
    assert_eq!(grade(60), 'D');
    assert_eq!(grade(0), 'F');
}

#[test]
fn vowels() {
    assert!(is_vowel('a'));
    assert!(is_vowel('o'));
    assert!(!is_vowel('b'));
    assert!(!is_vowel('y'));
}

#[test]
fn classes() {
    assert_eq!(classify(0), "zero");
    assert_eq!(classify(-3), "negative");
    assert_eq!(classify(7), "positive");
}
