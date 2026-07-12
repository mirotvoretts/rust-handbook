//! 01 (0x) - функции, обобщённые по длине массива. Эталонное решение.
pub fn sum<const N: usize>(arr: [i32; N]) -> i32 {
    let mut total = 0;
    for x in arr {
        total += x;
    }
    total
}

pub fn dot<const N: usize>(a: [i32; N], b: [i32; N]) -> i32 {
    let mut total = 0;
    for i in 0..N {
        total += a[i] * b[i];
    }
    total
}
