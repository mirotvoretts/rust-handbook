//! 02 (0x) - `hashmap!{ k => v, ... }`: синхронное повторение пар. Эталонное решение.

#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    ($($k:expr => $v:expr),+ $(,)?) => {{
        let mut m = ::std::collections::HashMap::new();
        $( m.insert($k, $v); )+
        m
    }};
}
