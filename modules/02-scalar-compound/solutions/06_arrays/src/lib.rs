//! 06 (1x) - Массивы `[T; N]`. Эталонное решение.

pub fn sum5(arr: [i32; 5]) -> i32 {
    let mut acc = 0;
    for x in arr {
        acc += x;
    }
    acc
}

pub fn reverse5(arr: [i32; 5]) -> [i32; 5] {
    [arr[4], arr[3], arr[2], arr[1], arr[0]]
}

pub fn contains5(arr: [i32; 5], x: i32) -> bool {
    for v in arr {
        if v == x {
            return true;
        }
    }
    false
}
