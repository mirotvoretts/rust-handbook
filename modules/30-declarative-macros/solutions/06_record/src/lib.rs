//! 06 (2x) - `record!(Name { field: ty, ... })`: генерация структуры. Эталонное решение.

#[macro_export]
macro_rules! record {
    ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            $( pub $field: $ty, )*
        }

        impl $name {
            pub fn new($($field: $ty),*) -> Self {
                $name { $($field),* }
            }
        }
    };
}
