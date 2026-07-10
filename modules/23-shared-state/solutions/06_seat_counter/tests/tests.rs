use sol_23_06_seat_counter::SeatCounter;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

#[test]
fn single_thread_fills_exactly() {
    let sc = SeatCounter::new(3);
    assert!(sc.try_take());
    assert!(sc.try_take());
    assert!(sc.try_take());
    assert!(!sc.try_take());
    assert_eq!(sc.taken(), 3);
}

#[test]
fn never_exceeds_capacity_under_contention() {
    let cap = 100;
    let sc = Arc::new(SeatCounter::new(cap));
    let successes = Arc::new(AtomicUsize::new(0));

    let mut hs = Vec::new();
    for _ in 0..8 {
        let sc = Arc::clone(&sc);
        let s = Arc::clone(&successes);
        hs.push(thread::spawn(move || {
            for _ in 0..1000 {
                if sc.try_take() {
                    s.fetch_add(1, Ordering::SeqCst);
                }
            }
        }));
    }
    for h in hs {
        h.join().unwrap();
    }

    assert_eq!(successes.load(Ordering::SeqCst), cap);
    assert_eq!(sc.taken(), cap);
}
