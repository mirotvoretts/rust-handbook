use sol_29_06_panic_boundary::*;

#[test]
fn division_ok() {
    let mut out = 0;
    let code = unsafe { checked_div(10, 2, &mut out) };
    assert_eq!(code, 0);
    assert_eq!(out, 5);
}

#[test]
fn panic_becomes_error_code() {
    // Глушим печать паники в stderr, чтобы вывод теста был чистым.
    std::panic::set_hook(Box::new(|_| {}));
    let mut out = -1;
    let code = unsafe { checked_div(10, 0, &mut out) };
    let _ = std::panic::take_hook();
    assert_eq!(code, 1);
    assert_eq!(out, -1); // при ошибке out не трогаем
}

#[test]
fn null_out_on_success() {
    // out == null при успехе: писать некуда, но паниковать нельзя.
    let code = unsafe { checked_div(8, 4, std::ptr::null_mut()) };
    assert_eq!(code, 0);
}
