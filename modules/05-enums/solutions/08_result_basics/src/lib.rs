//! 08 (1x) - `Result` с `Ok`/`Err`. Эталонное решение.

pub fn checked_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

pub fn parse_bool(s: &str) -> Result<bool, String> {
    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!("invalid bool: {s}")),
    }
}
