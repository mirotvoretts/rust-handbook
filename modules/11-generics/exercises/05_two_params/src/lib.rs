//! 05 (1x) — Обобщённый enum с двумя параметрами.
//!
//! `Either<L, R>` — «или одно, или другое», как Result без смысловой нагрузки
//! «успех/ошибка». Реализуйте методы. Заметьте: swap меняет типы местами в СИГНАТУРЕ —
//! Either<L, R> -> Either<R, L>.

/// Либо Left(L), либо Right(R).
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    /// true, если это Left.
    pub fn is_left(&self) -> bool {
        todo!("match или matches!")
    }

    /// Забирает L, если это Left.
    pub fn into_left(self) -> Option<L> {
        todo!()
    }

    /// Забирает R, если это Right.
    pub fn into_right(self) -> Option<R> {
        todo!()
    }

    /// Меняет стороны местами: Left(l) -> Right(l), Right(r) -> Left(r).
    pub fn swap(self) -> Either<R, L> {
        todo!()
    }
}
