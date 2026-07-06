//! 09 (2x) - `while let`: вычерпывание `Vec`. Эталонное решение.

pub fn pop_all(stack: &mut Vec<i32>) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(x) = stack.pop() {
        out.push(x);
    }
    out
}

pub fn sum_drain(stack: &mut Vec<i32>) -> i32 {
    let mut acc = 0;
    while let Some(x) = stack.pop() {
        acc += x;
    }
    acc
}
