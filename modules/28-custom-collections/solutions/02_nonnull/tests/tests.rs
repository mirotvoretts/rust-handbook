use sol_28_02_nonnull::*;

#[test]
fn empty_slice_has_no_first() {
    let mut empty: [u32; 0] = [];
    assert!(first_ptr(&mut empty).is_none());
}

#[test]
fn read_and_offset() {
    let mut arr = [10u32, 20, 30];
    let p = first_ptr(&mut arr).unwrap();
    unsafe {
        assert_eq!(read_at(p), 10);
        let p2 = offset(p, 2);
        assert_eq!(read_at(p2), 30);
    }
}

#[test]
fn write_through_pointer() {
    let mut arr = [0u32; 3];
    let p = first_ptr(&mut arr).unwrap();
    unsafe {
        write_at(p, 7);
        let p1 = offset(p, 1);
        write_at(p1, 8);
    }
    assert_eq!(arr, [7, 8, 0]);
}
