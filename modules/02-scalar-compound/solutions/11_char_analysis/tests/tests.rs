use sol_02_11_char_analysis::{caesar_shift, count_alpha_digit};

#[test]
fn classification() {
    assert_eq!(count_alpha_digit("ab12c"), (3, 2));
    assert_eq!(count_alpha_digit(""), (0, 0));
    assert_eq!(count_alpha_digit("!!!"), (0, 0));
    assert_eq!(count_alpha_digit("привет123"), (6, 3));
}

#[test]
fn caesar() {
    assert_eq!(caesar_shift('a', 1), 'b');
    assert_eq!(caesar_shift('z', 1), 'a');
    assert_eq!(caesar_shift('A', 2), 'C');
    assert_eq!(caesar_shift('Z', 1), 'A');
    assert_eq!(caesar_shift('!', 5), '!');
    assert_eq!(caesar_shift('a', 26), 'a');
    assert_eq!(caesar_shift('a', 27), 'b');
}
