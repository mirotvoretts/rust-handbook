//! 09 (2x) — Владение и два параметра типа. Эталонное решение.

/// [a1,a2,...] + [b1,b2,...] -> [(a1,b1), (a2,b2), ...]; длина = min(len_a, len_b).
pub fn zip_pairs<A, B>(xs: Vec<A>, ys: Vec<B>) -> Vec<(A, B)> {
    xs.into_iter().zip(ys).collect()
}

/// [(a1,b1), ...] -> ([a1,...], [b1,...]).
pub fn unzip_pairs<A, B>(pairs: Vec<(A, B)>) -> (Vec<A>, Vec<B>) {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (a, b) in pairs {
        xs.push(a);
        ys.push(b);
    }
    (xs, ys)
}
