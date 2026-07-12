//! 01 (0x) - `my_vec![...]`: повторение и хвостовая запятая. Эталонное решение.

#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($($x:expr),+ $(,)?) => {{
        let mut v = Vec::new();
        $( v.push($x); )+
        v
    }};
}
