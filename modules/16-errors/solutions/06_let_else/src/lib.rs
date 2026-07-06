//! 06 (1x) - let else. Эталонное решение.

/// Первое число из строки вида "count: 42; ...". Нет числа -> "n/a".
pub fn extract_count(line: &str) -> String {
    let Some(rest) = line.strip_prefix("count: ") else {
        return String::from("n/a");
    };
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    let Ok(n) = digits.parse::<u64>() else {
        return String::from("n/a");
    };
    format!("count={n}")
}

/// Сумма первых двух элементов; меньше двух -> 0.
pub fn sum_first_two(xs: &[i32]) -> i32 {
    let Some([a, b]) = xs.get(0..2).map(|s| [s[0], s[1]]) else {
        return 0;
    };
    a + b
}
