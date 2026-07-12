use ex_32_06_builder_typestate::{Request, RequestBuilder};

#[test]
fn builds_with_url() {
    let r = RequestBuilder::new()
        .method("POST")
        .url("http://x")
        .body("data")
        .build();
    assert_eq!(
        r,
        Request {
            url: "http://x".to_string(),
            method: "POST".to_string(),
            body: "data".to_string(),
        }
    );
}

#[test]
fn defaults_applied() {
    let r = RequestBuilder::new().url("http://y").build();
    assert_eq!(r.method, "GET");
    assert_eq!(r.body, "");
}

// Не компилируется - url обязателен:
// RequestBuilder::new().build();
// RequestBuilder::new().method("X").build();
