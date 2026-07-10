use ex_23_07_poison_recover::recover_sum;
use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn reads_clean_lock() {
    let m = Mutex::new(vec![1, 2, 3, 4]);
    assert_eq!(recover_sum(&m), 10);
}

#[test]
fn empty_vec() {
    let m = Mutex::new(Vec::<i64>::new());
    assert_eq!(recover_sum(&m), 0);
}

#[test]
fn reads_through_poison() {
    let m = Arc::new(Mutex::new(vec![10, 20, 30]));
    let m2 = Arc::clone(&m);

    // поток паникует под замком -> замок отравлен
    let res = thread::spawn(move || {
        let _g = m2.lock().unwrap();
        panic!("poison the lock");
    })
    .join();
    assert!(res.is_err());

    // подтверждаем: обычный lock теперь даёт Err
    assert!(m.lock().is_err());

    // а recover_sum всё равно читает данные
    assert_eq!(recover_sum(&m), 60);
    // и повторно тоже
    assert_eq!(recover_sum(&m), 60);
}
