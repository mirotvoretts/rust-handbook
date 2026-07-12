use ex_31_05_wrap_result::wrap_result;

#[wrap_result]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wrap_result]
fn greet() -> String {
    "hi".to_string()
}

#[test]
fn wraps_return_in_ok() {
    assert_eq!(add(2, 3), Ok::<i32, String>(5));
    assert_eq!(greet(), Ok("hi".to_string()));
}
