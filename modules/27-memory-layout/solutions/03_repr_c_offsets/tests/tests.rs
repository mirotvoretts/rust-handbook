use sol_27_03_repr_c_offsets::*;

#[test]
fn offsets() {
    // tag@0; id выровнен по 4 -> @4 (3 байта padding); flags@8; count@10.
    assert_eq!(field_offsets(), [0, 4, 8, 10]);
}

#[test]
fn size() {
    // 11 байт данных, округлённые вверх до выравнивания 4.
    assert_eq!(header_size(), 12);
}

#[test]
fn align() {
    assert_eq!(header_align(), 4);
}
