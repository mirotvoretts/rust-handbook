//! 08 (2x) - ПОЧИНИ СЛОМАННОЕ: несколько ошибок сразу. Эталонное решение.
//!
//! Исправления: `let mut total` (E0384) и убрали `;` после `total` в конце (E0308).

pub fn sum_to(n: i32) -> i32 {
    let mut total = 0;
    let mut i = 1;
    while i <= n {
        total += i;
        i += 1;
    }
    total
}
