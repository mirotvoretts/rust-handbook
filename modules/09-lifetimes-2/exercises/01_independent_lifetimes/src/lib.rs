//! 01 (0x) — ДОБАВЬТЕ ВРЕМЕНА ЖИЗНИ САМИ.
//!
//! Крейт НЕ КОМПИЛИРУЕТСЯ: `error[E0106]: missing lifetime specifier`. Функция возвращает
//! `first`, значит результат связан ТОЛЬКО с ним. Свяжите время жизни `'a` с `first` и
//! результатом, а `second` оставьте с независимым (элидированным) временем жизни:
//!   `pub fn keep_first<'a>(first: &'a str, second: &str) -> &'a str`.
//!
//! Смысл: `second` не обязан переживать результат — он может быть короткоживущим (это проверяет
//! тест `second_may_be_short_lived`).

pub fn keep_first(first: &str, second: &str) -> &str {
    first
}
