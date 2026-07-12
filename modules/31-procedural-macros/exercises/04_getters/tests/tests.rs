use ex_31_04_getters::Getters;

#[derive(Getters)]
struct Config {
    name: String,
    retries: u32,
}

#[test]
fn generates_getters() {
    let c = Config { name: "srv".to_string(), retries: 3 };
    assert_eq!(c.get_name().as_str(), "srv");
    assert_eq!(*c.get_retries(), 3);
}
