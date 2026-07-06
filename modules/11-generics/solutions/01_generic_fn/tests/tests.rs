use sol_11_01_generic_fn::{first, identity, last};

#[test]
fn identity_various_types() {
    assert_eq!(identity(5), 5);
    assert_eq!(identity("hi"), "hi");
    assert_eq!(identity(String::from("own")), "own");
    assert_eq!(identity((1, 2.5)), (1, 2.5));
}

#[test]
fn first_and_last() {
    let xs = [10, 20, 30];
    assert_eq!(first(&xs), Some(&10));
    assert_eq!(last(&xs), Some(&30));
}

#[test]
fn empty_slice() {
    let xs: [i32; 0] = [];
    assert_eq!(first(&xs), None);
    assert_eq!(last(&xs), None);
}

#[test]
fn works_for_strings_too() {
    let words = ["alpha", "beta"];
    assert_eq!(first(&words), Some(&"alpha"));
    assert_eq!(last(&words), Some(&"beta"));
}
