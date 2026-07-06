//! 07 (1x) - Комбинаторы `Option`. Эталонное решение.

pub fn double_opt(opt: Option<i32>) -> Option<i32> {
    opt.map(|x| x * 2)
}

pub fn or_zero(opt: Option<i32>) -> i32 {
    opt.unwrap_or(0)
}

pub fn keep_positive(opt: Option<i32>) -> Option<i32> {
    opt.filter(|&x| x > 0)
}
