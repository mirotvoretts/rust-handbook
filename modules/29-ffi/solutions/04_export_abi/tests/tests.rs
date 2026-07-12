use sol_29_04_export_abi::*;

#[test]
fn add_basic() {
    assert_eq!(add_i32(2, 3), 5);
    assert_eq!(add_i32(-4, 10), 6);
}

#[test]
fn add_wraps() {
    assert_eq!(add_i32(i32::MAX, 1), i32::MIN);
}

#[test]
fn sum_slice() {
    let data = [1i32, 2, 3, 4, -5];
    let s = unsafe { sum_array(data.as_ptr(), data.len()) };
    assert_eq!(s, 5);
}

#[test]
fn sum_null_is_zero() {
    assert_eq!(unsafe { sum_array(std::ptr::null(), 0) }, 0);
}
