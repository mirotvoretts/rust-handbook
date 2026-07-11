use ex_27_01_layout_facts::*;

#[test]
fn u64_facts() {
    assert_eq!(u64_layout(), (8, 8));
}

#[test]
fn pair_facts() {
    // (u8, u32): 1 байт данных + 3 padding + 4 байта, выравнивание по u32.
    assert_eq!(pair_layout(), (8, 4));
}

#[test]
fn pair_padding_is_three() {
    assert_eq!(pair_padding(), 3);
}

#[test]
fn array_facts() {
    // [u16; 5]: 5 элементов по 2 байта, выравнивание u16.
    assert_eq!(array_layout(), (10, 2));
}
