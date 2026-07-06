use sol_03_12_bubble_sort::{bubble_sort, is_sorted_asc};

#[test]
fn sorted_check() {
    assert!(is_sorted_asc(&[]));
    assert!(is_sorted_asc(&[1]));
    assert!(is_sorted_asc(&[1, 2, 2, 3]));
    assert!(!is_sorted_asc(&[2, 1]));
    assert!(!is_sorted_asc(&[1, 3, 2]));
}

#[test]
fn sorting() {
    let mut a = [3, 1, 2];
    bubble_sort(&mut a);
    assert_eq!(a, [1, 2, 3]);

    let mut b = [5, 4, 3, 2, 1];
    bubble_sort(&mut b);
    assert_eq!(b, [1, 2, 3, 4, 5]);

    let mut c = [1, 2, 3];
    bubble_sort(&mut c);
    assert_eq!(c, [1, 2, 3]);

    let mut d = [4, 2, 4, 1, 2];
    bubble_sort(&mut d);
    assert_eq!(d, [1, 2, 2, 4, 4]);
    assert!(is_sorted_asc(&d));

    let mut e: [i32; 0] = [];
    bubble_sort(&mut e);
    assert_eq!(e, []);
}
