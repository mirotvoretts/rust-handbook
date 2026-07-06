use ex_19_06_lazy_toolbox::{
    flatten_rows, indexed_chars, pairwise_sum, reversed, running_sum, take_below,
};

#[test]
fn enumerate_chars() {
    assert_eq!(indexed_chars("ab"), vec![(0, 'a'), (1, 'b')]);
    assert_eq!(indexed_chars(""), Vec::<(usize, char)>::new());
}

#[test]
fn zip_pairwise() {
    assert_eq!(pairwise_sum(&[1, 2, 3], &[10, 20, 30]), vec![11, 22, 33]);
    assert_eq!(pairwise_sum(&[1, 2, 3], &[10, 20]), vec![11, 22]); // стоп по короткому
}

#[test]
fn rev_values() {
    assert_eq!(reversed(&[1, 2, 3]), vec![3, 2, 1]);
    assert_eq!(reversed(&[]), Vec::<i32>::new());
}

#[test]
fn take_while_below_limit() {
    assert_eq!(take_below(&[1, 2, 3, 9, 2, 1], 5), vec![1, 2, 3]); // на 9 обрыв, хвост игнор
    assert_eq!(take_below(&[9, 1], 5), Vec::<i32>::new());
}

#[test]
fn flatten_nested() {
    let rows = vec![vec![1, 2], vec![], vec![3, 4, 5]];
    assert_eq!(flatten_rows(&rows), vec![1, 2, 3, 4, 5]);
}

#[test]
fn scan_running_sum() {
    assert_eq!(running_sum(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
    assert_eq!(running_sum(&[]), Vec::<i64>::new());
}
