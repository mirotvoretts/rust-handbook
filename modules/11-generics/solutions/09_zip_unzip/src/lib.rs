//! 09 (2x) — Владение и два параметра типа. Эталонное решение.

/// [a1,a2,...] + [b1,b2,...] -> [(a1,b1), (a2,b2), ...]; длина = min(len_a, len_b).
pub fn zip_pairs<A, B>(xs: Vec<A>, ys: Vec<B>) -> Vec<(A, B)> {
    let mut out = Vec::new();
    let mut it_a = xs.into_iter();
    let mut it_b = ys.into_iter();
    loop {
        match (it_a.next(), it_b.next()) {
            (Some(a), Some(b)) => out.push((a, b)),
            _ => break,
        }
    }
    out
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
