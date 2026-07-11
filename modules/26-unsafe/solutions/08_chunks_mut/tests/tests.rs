use sol_26_08_chunks_mut::chunks_mut;

#[test]
fn even_split() {
    let mut data = [1, 2, 3, 4, 5, 6];
    let chunks = chunks_mut(&mut data, 2);
    assert_eq!(chunks.len(), 3);
    assert_eq!(chunks[0], &mut [1, 2]);
    assert_eq!(chunks[1], &mut [3, 4]);
    assert_eq!(chunks[2], &mut [5, 6]);
}

#[test]
fn remainder_last_chunk_shorter() {
    let mut data = [1, 2, 3, 4, 5, 6, 7];
    let chunks = chunks_mut(&mut data, 3);
    assert_eq!(chunks.len(), 3);
    assert_eq!(chunks[0], &mut [1, 2, 3]);
    assert_eq!(chunks[1], &mut [4, 5, 6]);
    assert_eq!(chunks[2], &mut [7]);
}

#[test]
fn all_chunks_mutable_simultaneously() {
    let mut data = [10, 20, 30, 40, 50];
    let chunks = chunks_mut(&mut data, 2);
    for chunk in &chunks {
        let _ = chunk.len();
    }
    let mut chunks = chunks;
    chunks[0][0] = 1;
    chunks[2][0] = 500;
    drop(chunks);
    assert_eq!(data, [1, 20, 30, 40, 500]);
}

#[test]
fn size_larger_than_len() {
    let mut data = [1, 2, 3];
    let chunks = chunks_mut(&mut data, 10);
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0], &mut [1, 2, 3]);
}

#[test]
fn empty_slice() {
    let mut data: [i32; 0] = [];
    let chunks = chunks_mut(&mut data, 3);
    assert!(chunks.is_empty());
}

#[test]
#[should_panic]
fn zero_size_panics() {
    let mut data = [1, 2, 3];
    let _ = chunks_mut(&mut data, 0);
}
