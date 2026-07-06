use sol_00_01_hello_cargo::greeting;

#[test]
fn says_hello() {
    assert_eq!(greeting(), "Hello, cargo!");
}
