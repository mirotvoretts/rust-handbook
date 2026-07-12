use ex_29_02_c_union::*;

#[test]
fn u_bits_as_f32() {
    // 0x3f800000 - это бит-паттерн f32 1.0.
    let b = from_u32(0x3f80_0000);
    assert_eq!(as_f32(&b), 1.0_f32);
}

#[test]
fn f32_as_u_bits() {
    let b = from_f32(1.0);
    assert_eq!(as_u32(&b), 0x3f80_0000);
}

#[test]
fn bytes_match_native() {
    let b = from_u32(0x0102_0304);
    assert_eq!(as_ne_bytes(&b), 0x0102_0304_u32.to_ne_bytes());
}
