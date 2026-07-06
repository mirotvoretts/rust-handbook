//! 04 (2x) - ДОБАВЬТЕ OUTLIVES-ОГРАНИЧЕНИЕ `'big: 'a` САМИ.
//!
//! Крейт НЕ КОМПИЛИРУЕТСЯ: "lifetime may not live long enough". Функция возвращает ссылку со
//! временем жизни `'big` как `&'a str`, но компилятор не знает, что `'big` не короче `'a` -
//! между именованными параметрами он outlives-связь не додумывает. Добавьте ограничение
//! `'big: 'a` (читается "`'big` живёт хотя бы `'a`"):
//!   `pub fn as_shorter<'a, 'big: 'a>(big: &'big str) -> &'a str`.

pub fn as_shorter<'a, 'big>(big: &'big str) -> &'a str {
    big
}
