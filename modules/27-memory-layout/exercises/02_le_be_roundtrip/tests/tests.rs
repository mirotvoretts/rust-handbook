use ex_27_02_le_be_roundtrip::*;

#[test]
fn to_bytes_order() {
    let n: u32 = 0x0A0B0C0D;
    assert_eq!(to_be(n), [0x0A, 0x0B, 0x0C, 0x0D]);
    assert_eq!(to_le(n), [0x0D, 0x0C, 0x0B, 0x0A]);
}

#[test]
fn from_bytes_order() {
    assert_eq!(from_be([0x0A, 0x0B, 0x0C, 0x0D]), 0x0A0B0C0D);
    assert_eq!(from_le([0x0D, 0x0C, 0x0B, 0x0A]), 0x0A0B0C0D);
}

#[test]
fn roundtrip() {
    for n in [0u32, 1, 255, 256, 0xDEAD_BEEF, u32::MAX] {
        assert_eq!(from_le(to_le(n)), n);
        assert_eq!(from_be(to_be(n)), n);
    }
}

#[test]
fn le_be_differ_for_multibyte() {
    let n = 0x0000_0001u32;
    assert_ne!(to_le(n), to_be(n));
}
