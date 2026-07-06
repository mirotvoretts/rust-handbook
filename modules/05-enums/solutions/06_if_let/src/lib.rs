//! 06 (1x) - `if let` для одного варианта. Эталонное решение.

pub fn label(opt: Option<i32>) -> String {
    if let Some(n) = opt {
        format!("value: {n}")
    } else {
        String::from("none")
    }
}

pub fn is_some_even(opt: Option<i32>) -> bool {
    if let Some(n) = opt {
        n % 2 == 0
    } else {
        false
    }
}
