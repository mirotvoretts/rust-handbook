//! 05 (2x) - упаковка и распаковка среза пикселей. Эталонное решение.

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// Срез пикселей -> поток байт.
pub fn pack(pixels: &[Rgba]) -> Vec<u8> {
    let mut out = Vec::with_capacity(pixels.len() * 4);
    for p in pixels {
        out.extend_from_slice(&[p.r, p.g, p.b, p.a]);
    }
    out
}

/// Поток байт -> вектор пикселей; `None`, если длина не кратна 4.
pub fn unpack(bytes: &[u8]) -> Option<Vec<Rgba>> {
    if !bytes.len().is_multiple_of(4) {
        return None;
    }
    let pixels = bytes
        .chunks_exact(4)
        .map(|c| Rgba {
            r: c[0],
            g: c[1],
            b: c[2],
            a: c[3],
        })
        .collect();
    Some(pixels)
}
