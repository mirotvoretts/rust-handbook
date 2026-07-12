//! 08 (3x) - `json!(...)`: рекурсивный макрос над мини-DSL (разделы 6, 3 README).
//!
//! Напиши макрос `json!`, строящий значение типа `Json` (задан ниже) из литерального
//! описания в стиле JSON:
//!
//! ```ignore
//! json!(null)                              == Json::Null
//! json!(true)                              == Json::Bool(true)
//! json!(42)                                == Json::Num(42.0)
//! json!("hi")                              == Json::Str("hi".into())
//! json!([1, 2])                            == Json::Array(vec![Json::Num(1.0), Json::Num(2.0)])
//! json!({ "a": 1, "b": [true, null] })     == вложенный Json::Object
//! ```
//!
//! Тип `Json` и конверсии `From<bool/f64/i32/&str>` уже даны - скалярные значения
//! превращаются в `Json` через `Json::from(...)`. Твоя задача - только макрос.
//!
//! Подсказки:
//! - несколько правил, пробуются сверху вниз; частные формы (null, `[...]`, `{...}`) ставь
//!   выше общего правила для скаляра;
//! - `null` матчится литеральным токеном; массив - образцом `[ $($elem:tt),* ]`, объект -
//!   `{ $($key:literal : $val:tt),* }`; элементы бери как `tt` и рекурсивно оборачивай
//!   `$crate::json!($elem)`;
//! - общее правило - `($other:expr) => { $crate::Json::from($other) }`;
//! - обязательно ссылайся на тип полным путём `$crate::Json`, иначе макрос сломается в
//!   чужом крейте (раздел 7).
//!
//! Заглушка выдаёт `compile_error!`, пока макрос не реализован. Похожий разбор - `json!` в
//! документации serde_json; рекурсия в макросах:
//! <https://veykril.github.io/tlborm/decl-macros/patterns/tt-muncher.html>.

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
    ($($t:tt)*) => {
        compile_error!("реализуй макрос json (упражнение не решено)")
    };
}
