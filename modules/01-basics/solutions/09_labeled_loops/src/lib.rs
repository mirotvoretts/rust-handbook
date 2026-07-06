//! 09 (2x) — Метки циклов и `break 'label`. Эталонное решение.

pub fn factor_pair(target: u32) -> Option<(u32, u32)> {
    let mut ans = None;
    'outer: for i in 2..target {
        for j in i..target {
            if i * j == target {
                ans = Some((i, j));
                break 'outer;
            }
        }
    }
    ans
}

pub fn pythagorean_triple(perimeter: u32) -> Option<(u32, u32, u32)> {
    let mut ans = None;
    'outer: for a in 1..perimeter {
        for b in (a + 1)..perimeter {
            if a + b >= perimeter {
                break; // c уже не будет положительным — к следующему a
            }
            let c = perimeter - a - b;
            if c > b && a * a + b * b == c * c {
                ans = Some((a, b, c));
                break 'outer;
            }
        }
    }
    ans
}
