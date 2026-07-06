use sol_10_06_pub_crate::config;

#[test]
fn effective_value() {
    assert_eq!(config::effective(Some(3)), 3);
    assert_eq!(config::effective(None), 7);
}
