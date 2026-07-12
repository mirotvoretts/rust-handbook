use sol_31_07_builder_optional::Builder;

#[derive(Builder)]
struct Server {
    host: String,
    port: u16,
    label: Option<String>,
}

#[test]
fn optional_can_be_omitted() {
    let s = Server::builder().host("h".to_string()).port(1).build().unwrap();
    assert_eq!(s.host, "h");
    assert_eq!(s.port, 1);
    assert_eq!(s.label, None);
}

#[test]
fn optional_can_be_set() {
    let s = Server::builder()
        .host("h".to_string())
        .port(1)
        .label("L".to_string())
        .build()
        .unwrap();
    assert_eq!(s.label, Some("L".to_string()));
}

#[test]
fn required_missing_errors() {
    let r = Server::builder().host("h".to_string()).build();
    assert!(r.is_err());
}
