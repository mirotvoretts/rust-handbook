use ex_14_05_dyn_compat::{fill, Buffer, VecBuffer};

#[test]
fn buffer_basics() {
    let mut b = VecBuffer { bytes: vec![] };
    b.push_byte(7);
    assert_eq!(b.len(), 1);
    assert_eq!(b.bytes, vec![7]);
}

#[test]
fn fresh_copy_still_callable_on_concrete_type() {
    let b = VecBuffer { bytes: vec![1, 2, 3] };
    let fresh = b.fresh_copy();
    assert_eq!(fresh.len(), 0);
    assert_eq!(b.len(), 3);
}

#[test]
fn dyn_usage_compiles_and_works() {
    let mut b = VecBuffer { bytes: vec![] };
    fill(&mut b, 4); // &mut dyn Buffer - трейт обязан быть dyn-совместимым
    assert_eq!(b.bytes, vec![0, 1, 2, 3]);
}
