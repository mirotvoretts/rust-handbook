use ex_19_02_into_iter_forms::{concat_owned, double_in_place, sum_borrowed};

#[test]
fn borrows_and_sums() {
    let v = vec![1, 2, 3, 4];
    assert_eq!(sum_borrowed(&v), 10);
    assert_eq!(v.len(), 4); // вектор цел после чтения
}

#[test]
fn mutates_in_place() {
    let mut v = vec![1, 2, 3];
    double_in_place(&mut v);
    assert_eq!(v, vec![2, 4, 6]);
}

#[test]
fn consumes_and_concats() {
    let v = vec![String::from("ab"), String::from("cd"), String::from("e")];
    assert_eq!(concat_owned(v), "abcde");
    assert_eq!(concat_owned(vec![]), "");
}
