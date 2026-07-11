use sol_27_05_pixel_bytes::*;

fn sample() -> Vec<Rgba> {
    vec![
        Rgba { r: 1, g: 2, b: 3, a: 4 },
        Rgba { r: 255, g: 0, b: 128, a: 64 },
    ]
}

#[test]
fn pack_layout() {
    let bytes = pack(&sample());
    assert_eq!(bytes, vec![1, 2, 3, 4, 255, 0, 128, 64]);
}

#[test]
fn empty() {
    assert_eq!(pack(&[]), Vec::<u8>::new());
    assert_eq!(unpack(&[]), Some(Vec::new()));
}

#[test]
fn roundtrip() {
    let p = sample();
    assert_eq!(unpack(&pack(&p)), Some(p));
}

#[test]
fn incomplete_pixel_is_none() {
    assert_eq!(unpack(&[1, 2, 3]), None);
    assert_eq!(unpack(&[1, 2, 3, 4, 5]), None);
}
