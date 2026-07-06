//! 04 (1x) - Потребители: fold / sum / find / any / all / max_by_key / position.
//!
//! Каждую функцию решай одним потребителем (плюс, где нужно, адаптером перед ним).

/// Сумма квадратов элементов. Тип суммы - i64.
pub fn sum_of_squares(v: &[i64]) -> i64 {
    todo!("v.iter().map(|x| x * x).sum()")
}

/// Мин и макс за ОДИН проход через fold (аккумулятор несёт кортеж).
/// Для пустого среза - None. Так видно, что fold - общая свёртка, а не только
/// sum/product: он тащит произвольное состояние.
pub fn min_max(v: &[i64]) -> Option<(i64, i64)> {
    todo!("v.iter().fold(None, |acc, &x| match acc { None => Some((x, x)), Some((lo, hi)) => Some((lo.min(x), hi.max(x))) })")
}

/// Первое число, чей квадрат строго больше threshold (или None).
pub fn first_square_over(v: &[i64], threshold: i64) -> Option<i64> {
    todo!("find по предикату, вернуть само число (copied)")
}

/// Есть ли отрицательное число.
pub fn has_negative(v: &[i64]) -> bool {
    todo!("any")
}

/// Все ли строго положительны (для пустого - true).
pub fn all_positive(v: &[i64]) -> bool {
    todo!("all")
}

/// Самое длинное слово (ПЕРВОЕ при равенстве длин), или None для пустого среза.
/// Ловушка: max_by_key при равенстве ключей берёт ПОСЛЕДНИЙ элемент, поэтому
/// "первое при равенстве" им не выразить - сверни через fold со строгим сравнением.
pub fn longest<'a>(words: &[&'a str]) -> Option<&'a str> {
    todo!("fold(None, |best, w| если w строго длиннее лучшего - w, иначе best)")
}

/// Индекс первого слова, равного target (или None).
pub fn index_of(words: &[&str], target: &str) -> Option<usize> {
    todo!("position(|w| *w == target)")
}
