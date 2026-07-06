//! 08 (2x) - Мутация без лишних ограничений. Эталонное решение.

/// Меняет местами первый и последний элементы. Для len < 2 - ничего не делает.
pub fn swap_ends<T>(xs: &mut [T]) {
    if xs.len() >= 2 {
        let last = xs.len() - 1;
        xs.swap(0, last);
    }
}

/// Кладёт new в слот, возвращает старое значение.
pub fn exchange<T>(slot: &mut T, new: T) -> T {
    std::mem::replace(slot, new)
}

/// Разворачивает срез на месте (первый <-> последний, второй <-> предпоследний, ...).
pub fn reverse_in_place<T>(xs: &mut [T]) {
    let mut i = 0;
    let mut j = xs.len().saturating_sub(1);
    while i < j {
        xs.swap(i, j);
        i += 1;
        j -= 1;
    }
}
