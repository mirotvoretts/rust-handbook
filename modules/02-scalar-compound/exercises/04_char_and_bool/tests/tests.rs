use ex_02_04_char_and_bool::{digit_value, is_vowel, xor};

#[test]
fn vowels() {
    assert!(is_vowel('a'));
    assert!(is_vowel('u'));
    assert!(!is_vowel('b'));
    assert!(!is_vowel('A')); // только строчные
}

#[test]
fn digits() {
    assert_eq!(digit_value('7'), Some(7));
    assert_eq!(digit_value('0'), Some(0));
    assert_eq!(digit_value('x'), None);
}

#[test]
fn xors() {
    assert!(!xor(true, true));
    assert!(xor(true, false));
    assert!(xor(false, true));
    assert!(!xor(false, false));
}
