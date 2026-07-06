use sol_17_05_vecdeque_window::SlidingAverage;

#[test]
fn grows_until_capacity() {
    let mut s = SlidingAverage::new(3);
    assert_eq!(s.push(3.0), 3.0);
    assert_eq!(s.push(5.0), 4.0);
    assert_eq!(s.push(7.0), 5.0);
    assert_eq!(s.len(), 3);
}

#[test]
fn evicts_oldest() {
    let mut s = SlidingAverage::new(2);
    s.push(10.0);
    s.push(20.0);
    assert_eq!(s.push(30.0), 25.0); // 10 вытеснено: (20+30)/2
    assert_eq!(s.len(), 2);
}
