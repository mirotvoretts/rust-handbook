use ex_01_09_labeled_loops::{factor_pair, pythagorean_triple};

#[test]
fn factor_pairs() {
    assert_eq!(factor_pair(12), Some((2, 6)));
    assert_eq!(factor_pair(9), Some((3, 3)));
    assert_eq!(factor_pair(4), Some((2, 2)));
    assert_eq!(factor_pair(7), None);
    assert_eq!(factor_pair(1), None);
    assert_eq!(factor_pair(0), None);
}

#[test]
fn triples() {
    assert_eq!(pythagorean_triple(12), Some((3, 4, 5)));
    assert_eq!(pythagorean_triple(1000), Some((200, 375, 425)));
    assert_eq!(pythagorean_triple(4), None);
}
