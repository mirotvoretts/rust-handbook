use ex_11_09_zip_unzip::{unzip_pairs, zip_pairs};

#[test]
fn zip_equal_lengths() {
    assert_eq!(zip_pairs(vec![1, 2], vec!['a', 'b']), vec![(1, 'a'), (2, 'b')]);
}

#[test]
fn zip_truncates_to_shorter() {
    assert_eq!(zip_pairs(vec![1, 2, 3], vec!["x"]), vec![(1, "x")]);
    assert_eq!(zip_pairs(Vec::<i32>::new(), vec![1, 2]), vec![]);
}

#[test]
fn owned_values_move_through() {
    let names = vec![String::from("a"), String::from("b")];
    let nums = vec![1, 2];
    let pairs = zip_pairs(names, nums);
    assert_eq!(pairs[0].0, "a");
}

#[test]
fn unzip_roundtrip() {
    let pairs = vec![(1, "one"), (2, "two"), (3, "three")];
    let (nums, words) = unzip_pairs(pairs);
    assert_eq!(nums, vec![1, 2, 3]);
    assert_eq!(words, vec!["one", "two", "three"]);
}
