use sol_30_08_json::{json, Json};

#[test]
fn scalars() {
    assert_eq!(json!(null), Json::Null);
    assert_eq!(json!(true), Json::Bool(true));
    assert_eq!(json!(false), Json::Bool(false));
    assert_eq!(json!(42), Json::Num(42.0));
    assert_eq!(json!("hi"), Json::Str("hi".to_string()));
}

#[test]
fn array() {
    assert_eq!(
        json!([1, 2, 3]),
        Json::Array(vec![Json::Num(1.0), Json::Num(2.0), Json::Num(3.0)])
    );
    assert_eq!(json!([]), Json::Array(vec![]));
}

#[test]
fn object() {
    let v = json!({ "a": 1, "b": 2 });
    let expected = Json::Object(vec![
        ("a".to_string(), Json::Num(1.0)),
        ("b".to_string(), Json::Num(2.0)),
    ]);
    assert_eq!(v, expected);
}

#[test]
fn nested() {
    let v = json!({ "name": "rust", "tags": [true, null] });
    let expected = Json::Object(vec![
        ("name".to_string(), Json::Str("rust".to_string())),
        (
            "tags".to_string(),
            Json::Array(vec![Json::Bool(true), Json::Null]),
        ),
    ]);
    assert_eq!(v, expected);
}
