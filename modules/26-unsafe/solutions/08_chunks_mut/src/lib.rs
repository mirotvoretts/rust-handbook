//! 08 (3x) - раздать непересекающиеся `&mut`-подсрезы одного среза. Эталонное решение.

use std::slice;

pub fn chunks_mut<T>(slice: &mut [T], size: usize) -> Vec<&mut [T]> {
    assert!(size > 0, "size должен быть > 0");

    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    let mut out = Vec::new();

    let mut start = 0;
    while start < len {
        let this = (len - start).min(size); // последний кусок может быть короче
        // SAFETY: [start, start+this) лежит в исходной аллокации; куски идут
        // подряд и не пересекаются, поэтому одновременные &mut на них корректны.
        // Время жизни срезов привязано к входному &mut через элизию.
        let chunk = unsafe { slice::from_raw_parts_mut(ptr.add(start), this) };
        out.push(chunk);
        start += this;
    }

    out
}
