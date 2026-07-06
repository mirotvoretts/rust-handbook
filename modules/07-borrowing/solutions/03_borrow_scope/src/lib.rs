//! 03 (0x) — Прочитать через `&`, затем поменять через `&mut` (NLL). Эталонное решение.

pub fn push_sum(xs: &mut Vec<i32>) {
    let sum: i32 = xs.iter().sum(); // общий заём завершается здесь
    xs.push(sum); // теперь можно изменять
}

pub fn push_len(xs: &mut Vec<i32>) {
    let len = xs.len() as i32;
    xs.push(len);
}
