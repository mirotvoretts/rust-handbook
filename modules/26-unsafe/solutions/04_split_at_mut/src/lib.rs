//! 04 (1x) - безопасная обёртка над `from_raw_parts_mut`. Эталонное решение.

use std::slice;

pub fn split_at_mut(slice: &mut [i64], mid: usize) -> (&mut [i64], &mut [i64]) {
    let len = slice.len();
    assert!(mid <= len, "mid ({mid}) вне длины среза ({len})");
    let ptr = slice.as_mut_ptr();

    // SAFETY: mid <= len проверено; обе части лежат в одной аллокации и не
    // пересекаются ([0, mid) и [mid, len)), поэтому одновременные &mut корректны.
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
