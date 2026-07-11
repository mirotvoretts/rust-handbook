//! 07 (3x) - zero-copy-приведение через `bytemuck` (раздел 7 README).
//!
//! `Sample` - "plain old data": `#[repr(C)]`, только целые поля, без padding (два поля
//! по 4 байта, размер ровно 8). Такой тип реализует `bytemuck::Pod`, поэтому его
//! байтовое представление совпадает с представлением в памяти, а перевод в байты и
//! обратно делается через `bytemuck` без ручного unsafe.
//!
//! Реализуй:
//! - `encode(samples)` -> `Vec<u8>`: байты всех `Sample` подряд, в native-порядке.
//!   Возьми zero-copy-срез `&[u8]` через `bytemuck::cast_slice` и скопируй его в `Vec`
//!   (`.to_vec()`).
//! - `decode(bytes)` -> `Option<Vec<Sample>>`: если длина `bytes` не кратна размеру
//!   `Sample`, вернуть `None`; иначе восстановить вектор. Байты из `Vec<u8>` могут быть
//!   невыровнены под `Sample`, поэтому НЕ используй `cast_slice` (он паникует на
//!   невыровненном входе) - используй копирующий `bytemuck::pod_collect_to_vec`.
//! - `total_value(bytes)` -> `Option<i64>`: сумма поля `value` всех записей (как `i64`),
//!   или `None`, если `bytes` не раскодируется.
//!
//! Требование: `decode(&encode(&s)) == Some(s)` (round-trip).
//!
//! Конструкции за пределами теории (крейт `bytemuck`):
//! - вывод Pod/Zeroable и cast_slice: https://docs.rs/bytemuck/latest/bytemuck/
//! - копирующее восстановление вектора: bytemuck::pod_collect_to_vec
//!   https://docs.rs/bytemuck/latest/bytemuck/fn.pod_collect_to_vec.html

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
pub struct Sample {
    pub t: u32,
    pub value: i32,
}

/// Срез `Sample` -> байты (native-порядок), с копированием в `Vec`.
pub fn encode(samples: &[Sample]) -> Vec<u8> {
    let _ = samples;
    todo!()
}

/// Байты -> вектор `Sample`; `None`, если длина не кратна `size_of::<Sample>()`.
pub fn decode(bytes: &[u8]) -> Option<Vec<Sample>> {
    let _ = bytes;
    todo!()
}

/// Сумма поля `value` всех записей как `i64`; `None`, если байты не раскодируются.
pub fn total_value(bytes: &[u8]) -> Option<i64> {
    let _ = bytes;
    todo!()
}
