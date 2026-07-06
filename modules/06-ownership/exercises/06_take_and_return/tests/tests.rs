use ex_06_06_take_and_return::Config;

#[test]
fn defaults() {
    let c = Config::new();
    assert_eq!(c.name, "");
    assert!(!c.verbose);
    assert_eq!(c.retries, 0);
}

#[test]
fn builder_chain() {
    let c = Config::new()
        .with_name("server")
        .with_verbose(true)
        .with_retries(3);
    assert_eq!(
        c,
        Config {
            name: String::from("server"),
            verbose: true,
            retries: 3,
        }
    );
}

#[test]
fn partial_chain_keeps_defaults() {
    let c = Config::default().with_retries(5);
    assert_eq!(c.name, "");
    assert!(!c.verbose);
    assert_eq!(c.retries, 5);
}
