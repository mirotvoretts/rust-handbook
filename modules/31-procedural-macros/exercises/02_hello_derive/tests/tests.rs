use ex_31_02_hello_derive::Hello;

#[derive(Hello)]
struct Widget;

#[derive(Hello)]
struct Gadget {
    _x: i32,
}

#[test]
fn greets_with_type_name() {
    assert_eq!(Widget.hello(), "Hello, Widget!");
    assert_eq!(Gadget { _x: 0 }.hello(), "Hello, Gadget!");
}
