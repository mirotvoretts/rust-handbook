use ex_29_05_opaque_handle::*;

#[test]
fn lifecycle() {
    let c = counter_new();
    assert!(!c.is_null());
    unsafe {
        assert_eq!(counter_value(c), 0);
        counter_add(c, 5);
        counter_add(c, 10);
        assert_eq!(counter_value(c), 15);
        counter_add(c, -3);
        assert_eq!(counter_value(c), 12);
        counter_free(c);
    }
}

#[test]
fn two_independent_handles() {
    let a = counter_new();
    let b = counter_new();
    unsafe {
        counter_add(a, 100);
        counter_add(b, 1);
        assert_eq!(counter_value(a), 100);
        assert_eq!(counter_value(b), 1);
        counter_free(a);
        counter_free(b);
    }
}

#[test]
fn null_is_tolerated() {
    unsafe {
        assert_eq!(counter_value(std::ptr::null()), 0);
        counter_add(std::ptr::null_mut(), 5);
        counter_free(std::ptr::null_mut());
    }
}
