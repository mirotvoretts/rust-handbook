use sol_31_06_builder::Builder;

#[derive(Builder)]
struct Server {
    host: String,
    port: u16,
}

#[test]
fn builds_when_all_set() {
    let s = Server::builder()
        .host("localhost".to_string())
        .port(8080)
        .build()
        .unwrap();
    assert_eq!(s.host, "localhost");
    assert_eq!(s.port, 8080);
}

#[test]
fn errors_when_missing() {
    let r = Server::builder().host("x".to_string()).build();
    assert!(r.is_err());
}
