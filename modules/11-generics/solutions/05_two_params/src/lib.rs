//! 05 (1x) — Обобщённый enum с двумя параметрами. Эталонное решение.

/// Либо Left(L), либо Right(R).
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    /// true, если это Left.
    pub fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }

    /// Забирает L, если это Left.
    pub fn into_left(self) -> Option<L> {
        match self {
            Either::Left(l) => Some(l),
            Either::Right(_) => None,
        }
    }

    /// Забирает R, если это Right.
    pub fn into_right(self) -> Option<R> {
        match self {
            Either::Left(_) => None,
            Either::Right(r) => Some(r),
        }
    }

    /// Меняет стороны местами: Left(l) -> Right(l), Right(r) -> Left(r).
    pub fn swap(self) -> Either<R, L> {
        match self {
            Either::Left(l) => Either::Right(l),
            Either::Right(r) => Either::Left(r),
        }
    }
}
