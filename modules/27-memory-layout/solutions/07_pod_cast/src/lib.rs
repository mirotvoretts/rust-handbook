//! 07 (3x) - zero-copy-приведение через `bytemuck`. Эталонное решение.

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
pub struct Sample {
    pub t: u32,
    pub value: i32,
}

/// Срез `Sample` -> байты, с копированием в `Vec`.
pub fn encode(samples: &[Sample]) -> Vec<u8> {
    // cast_slice даёт zero-copy-вид &[u8]; to_vec копирует его во владеемый Vec.
    bytemuck::cast_slice::<Sample, u8>(samples).to_vec()
}

/// Байты -> вектор `Sample`; `None`, если длина не кратна `size_of::<Sample>()`.
pub fn decode(bytes: &[u8]) -> Option<Vec<Sample>> {
    if !bytes.len().is_multiple_of(std::mem::size_of::<Sample>()) {
        return None;
    }
    // pod_collect_to_vec копирует байты поэлементно, поэтому выравнивание входа не важно.
    Some(bytemuck::pod_collect_to_vec(bytes))
}

/// Сумма поля `value` всех записей как `i64`.
pub fn total_value(bytes: &[u8]) -> Option<i64> {
    let samples = decode(bytes)?;
    Some(samples.iter().map(|s| s.value as i64).sum())
}
