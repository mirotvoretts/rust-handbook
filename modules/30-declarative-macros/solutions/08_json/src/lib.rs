//! 08 (3x) - `json!(...)`: рекурсивный макрос над мини-DSL. Эталонное решение.

#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl From<bool> for Json {
    fn from(b: bool) -> Self {
        Json::Bool(b)
    }
}
impl From<f64> for Json {
    fn from(n: f64) -> Self {
        Json::Num(n)
    }
}
impl From<i32> for Json {
    fn from(n: i32) -> Self {
        Json::Num(n as f64)
    }
}
impl From<&str> for Json {
    fn from(s: &str) -> Self {
        Json::Str(s.to_string())
    }
}

#[macro_export]
macro_rules! json {
    (null) => {
        $crate::Json::Null
    };
    ([ $($elem:tt),* $(,)? ]) => {
        $crate::Json::Array(vec![ $( $crate::json!($elem) ),* ])
    };
    ({ $($key:literal : $val:tt),* $(,)? }) => {
        $crate::Json::Object(vec![ $( ($key.to_string(), $crate::json!($val)) ),* ])
    };
    ($other:expr) => {
        $crate::Json::from($other)
    };
}
