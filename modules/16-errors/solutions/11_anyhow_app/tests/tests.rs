use sol_16_11_anyhow_app::{mean_of_line, mean_per_line};

#[test]
fn means() {
    assert_eq!(mean_of_line("1 2 3").unwrap(), 2.0);
    assert_eq!(mean_of_line("10").unwrap(), 10.0);
}

#[test]
fn empty_line_is_error() {
    assert!(mean_of_line("   ").is_err());
}

#[test]
fn context_contains_token() {
    let err = mean_of_line("1 abc 3").unwrap_err();
    let chain = format!("{err:#}");
    assert!(chain.contains("abc"), "нет токена в цепочке: {chain}");
}

#[test]
fn per_line_adds_line_context() {
    let err = mean_per_line(&["1 2", "x"]).unwrap_err();
    let chain = format!("{err:#}");
    assert!(chain.contains("строка 1"), "нет номера строки: {chain}");
}
