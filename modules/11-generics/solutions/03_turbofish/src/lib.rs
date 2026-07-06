//! 03 (0x) - Turbofish и вывод типа результата. Эталонное решение.

/// Парсит каждую строку как i32; непарсящиеся строки пропускает.
pub fn parse_all(strs: &[&str]) -> Vec<i32> {
    let mut out = Vec::new();
    for s in strs {
        if let Ok(n) = s.parse::<i32>() {
            out.push(n);
        }
    }
    out
}

/// Числа от 1 до n включительно, собранные в вектор.
pub fn one_to_n(n: u32) -> Vec<u32> {
    (1..=n).collect::<Vec<u32>>()
}

/// Сумма всех корректных i32 в строках (пустой ввод -> 0).
pub fn sum_parsed(strs: &[&str]) -> i64 {
    let mut sum: i64 = 0;
    for n in parse_all(strs) {
        sum += n as i64;
    }
    sum
}
