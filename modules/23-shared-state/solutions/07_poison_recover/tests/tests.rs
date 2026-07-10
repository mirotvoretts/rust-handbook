use sol_23_07_poison_recover::recover_sum;
use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn reads_clean_lock() {
    let m = Mutex::new(vec![1, 2, 3, 4]);
    assert_eq!(recover_sum(&m), 10);
}

#[test]
fn reads_through_poison() {
    let m = Arc::new(Mutex::new(vec![10, 20, 30]));
    let m2 = Arc::clone(&m);
    let res = thread::spawn(move || {
        let _g = m2.lock().unwrap();
        panic!("poison the lock");
    })
    .join();
    assert!(res.is_err());
    assert!(m.lock().is_err());
    assert_eq!(recover_sum(&m), 60);
}
