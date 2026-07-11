//! 05 (2x) - построение массива через `MaybeUninit`. Эталонное решение.

use std::mem::{self, MaybeUninit};

pub fn labels() -> [String; 5] {
    // массив неинициализированных ячеек создаётся сразу: сам он init не требует.
    let mut buf = [const { MaybeUninit::<String>::uninit() }; 5];

    for (i, slot) in buf.iter_mut().enumerate() {
        slot.write(format!("item-{i}")); // заполняем ячейку, без дропа старого
    }

    // SAFETY: все 5 ячеек записаны выше; [MaybeUninit<String>; 5] и [String; 5]
    // имеют одинаковый размер и раскладку, transmute корректен.
    unsafe { mem::transmute::<[MaybeUninit<String>; 5], [String; 5]>(buf) }
}
