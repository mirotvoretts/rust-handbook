use std::error::Error;
use sol_16_10_thiserror::{get_number, ConfigError};

#[test]
fn finds_and_parses() {
    assert_eq!(get_number(&["a=1", "b=2"], "b").unwrap(), 2);
}

#[test]
fn missing_key() {
    match get_number(&["a=1"], "z") {
        Err(ConfigError::MissingKey(k)) => assert_eq!(k, "z"),
        other => panic!("ожидали MissingKey, получили {other:?}"),
    }
}

#[test]
fn derive_generated_everything() {
    let err = get_number(&["a=x"], "a").unwrap_err();
    assert!(matches!(err, ConfigError::BadNumber(_)));
    assert!(!err.to_string().is_empty());  // Display - из #[error]
    assert!(err.source().is_some());       // source - из #[from]
}
