use sol_02_12_run_length::{run_length_decode, run_length_encode};

#[test]
fn encoding() {
    assert_eq!(run_length_encode(&[]), Vec::<(u8, usize)>::new());
    assert_eq!(run_length_encode(&[5]), vec![(5, 1)]);
    assert_eq!(
        run_length_encode(&[1, 1, 2, 3, 3, 3]),
        vec![(1, 2), (2, 1), (3, 3)]
    );
    assert_eq!(run_length_encode(&[7, 7, 7]), vec![(7, 3)]);
}

#[test]
fn decoding() {
    assert_eq!(run_length_decode(&[]), Vec::<u8>::new());
    assert_eq!(run_length_decode(&[(1, 2), (2, 1)]), vec![1, 1, 2]);
    assert_eq!(run_length_decode(&[(9, 3)]), vec![9, 9, 9]);
}

#[test]
fn round_trip() {
    let data = vec![4, 4, 4, 1, 2, 2, 9];
    assert_eq!(run_length_decode(&run_length_encode(&data)), data);
}
