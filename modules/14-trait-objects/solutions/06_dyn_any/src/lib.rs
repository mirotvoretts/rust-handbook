//! 06 (2x) - dyn Any и downcast. Эталонное решение.

use std::any::Any;

pub struct Sorted {
    pub int_sum: i64,
    pub strings_joined: String,
    pub unknown_count: usize,
}

/// Разбирает мешок: i32 суммируются, String склеиваются, прочее считается.
pub fn sort_bag(bag: Vec<Box<dyn Any>>) -> Sorted {
    let mut result = Sorted {
        int_sum: 0,
        strings_joined: String::new(),
        unknown_count: 0,
    };
    for item in &bag {
        if let Some(n) = item.downcast_ref::<i32>() {
            result.int_sum += *n as i64;
        } else if let Some(s) = item.downcast_ref::<String>() {
            result.strings_joined.push_str(s);
        } else {
            result.unknown_count += 1;
        }
    }
    result
}
