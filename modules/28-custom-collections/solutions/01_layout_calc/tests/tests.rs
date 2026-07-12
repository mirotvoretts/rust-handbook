use sol_28_01_layout_calc::*;

#[test]
fn bytes_basic() {
    assert_eq!(array_bytes::<u32>(4), Some(16));
    assert_eq!(array_bytes::<u8>(10), Some(10));
    assert_eq!(array_bytes::<u64>(0), Some(0));
}

#[test]
fn layout_size_and_align() {
    assert_eq!(layout_of::<u16>(3), Some((6, 2)));
    assert_eq!(layout_of::<u64>(2), Some((16, 8)));
    // [u8; 5]: 5 байт, выравнивание 1.
    assert_eq!(layout_of::<u8>(5), Some((5, 1)));
}

#[test]
fn overflow_is_none() {
    // n * size_of::<u64>() заведомо переполняет предел isize::MAX.
    assert_eq!(array_bytes::<u64>(usize::MAX), None);
    assert_eq!(layout_of::<u64>(usize::MAX), None);
}

#[test]
fn stride() {
    assert_eq!(elem_stride::<u8>(), 1);
    assert_eq!(elem_stride::<u32>(), 4);
    assert_eq!(elem_stride::<(u8, u32)>(), 8);
}
