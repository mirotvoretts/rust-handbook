use ex_16_07_from_conversions::{load_port, ConfigError};

#[test]
fn happy_path() {
    assert_eq!(load_port(&["host=x", "port=8080"]), Ok(8080));
}

#[test]
fn missing_key_converted() {
    assert_eq!(
        load_port(&["host=x"]),
        Err(ConfigError::MissingKey(String::from("port")))
    );
}

#[test]
fn bad_number_converted() {
    match load_port(&["port=99999"]) {
        Err(ConfigError::BadNumber(_)) => {}
        other => panic!("ожидали BadNumber, получили {other:?}"),
    }
    match load_port(&["port=abc"]) {
        Err(ConfigError::BadNumber(_)) => {}
        other => panic!("ожидали BadNumber, получили {other:?}"),
    }
}
