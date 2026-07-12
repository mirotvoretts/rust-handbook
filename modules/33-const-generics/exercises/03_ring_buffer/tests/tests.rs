use ex_33_03_ring_buffer::RingBuffer;

#[test]
fn push_pop_fifo() {
    let mut rb: RingBuffer<i32, 3> = RingBuffer::new();
    assert_eq!(rb.capacity(), 3);
    assert!(rb.is_empty());
    rb.push(1);
    rb.push(2);
    assert_eq!(rb.len(), 2);
    assert_eq!(rb.pop(), Some(1));
    assert_eq!(rb.pop(), Some(2));
    assert_eq!(rb.pop(), None);
}

#[test]
fn overwrites_when_full() {
    let mut rb: RingBuffer<i32, 3> = RingBuffer::new();
    rb.push(1);
    rb.push(2);
    rb.push(3);
    rb.push(4);
    assert_eq!(rb.len(), 3);
    assert_eq!(rb.pop(), Some(2));
    assert_eq!(rb.pop(), Some(3));
    assert_eq!(rb.pop(), Some(4));
}
