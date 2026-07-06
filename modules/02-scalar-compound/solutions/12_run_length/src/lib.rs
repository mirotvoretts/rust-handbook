//! 12 (3x) - Кодирование по длинам серий (RLE). Эталонное решение.

pub fn run_length_encode(xs: &[u8]) -> Vec<(u8, usize)> {
    let mut out: Vec<(u8, usize)> = Vec::new();
    for &x in xs {
        match out.last_mut() {
            Some((val, count)) if *val == x => *count += 1,
            _ => out.push((x, 1)),
        }
    }
    out
}

pub fn run_length_decode(runs: &[(u8, usize)]) -> Vec<u8> {
    let mut out = Vec::new();
    for &(val, count) in runs {
        for _ in 0..count {
            out.push(val);
        }
    }
    out
}
