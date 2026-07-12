use ex_33_05_const_table::{build_pow2, popcount, POPCOUNT, POW2};

#[test]
fn powers() {
    assert_eq!(POW2, [1, 2, 4, 8, 16, 32, 64, 128]);
    const P: [u64; 4] = build_pow2::<4>();
    assert_eq!(P, [1, 2, 4, 8]);
}

#[test]
fn popcounts() {
    assert_eq!(popcount(0b1011), 3);
    assert_eq!(POPCOUNT[0], 0);
    assert_eq!(POPCOUNT[255], 8);
    assert_eq!(POPCOUNT[0b1010_1010], 4);
}
