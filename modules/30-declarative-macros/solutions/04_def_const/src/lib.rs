//! 04 (1x) - `def_const!(NAME: ty = expr)`: генерация элемента. Эталонное решение.

#[macro_export]
macro_rules! def_const {
    ($name:ident : $ty:ty = $val:expr) => {
        pub const $name: $ty = $val;
    };
}
