use ex_23_04_rwlock_registry::Registry;
use std::sync::Arc;
use std::thread;

#[test]
fn single_thread_basics() {
    let r = Registry::new();
    assert_eq!(r.get("a"), None);
    r.incr("a", 5);
    r.incr("a", 3);
    r.incr("b", 10);
    assert_eq!(r.get("a"), Some(8));
    assert_eq!(r.get("b"), Some(10));
    assert_eq!(r.get("missing"), None);
    assert_eq!(r.sum(), 18);
}

#[test]
fn concurrent_same_key() {
    let r = Arc::new(Registry::new());
    let mut hs = Vec::new();
    for _ in 0..8 {
        let r = Arc::clone(&r);
        hs.push(thread::spawn(move || {
            for _ in 0..1000 {
                r.incr("hits", 1);
            }
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
    assert_eq!(r.get("hits"), Some(8000));
    assert_eq!(r.sum(), 8000);
}

#[test]
fn concurrent_distinct_keys() {
    let r = Arc::new(Registry::new());
    let mut hs = Vec::new();
    for i in 0..10 {
        let r = Arc::clone(&r);
        hs.push(thread::spawn(move || {
            r.incr(&format!("k{i}"), i as i64);
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
    assert_eq!(r.sum(), (0..10).sum());
    assert_eq!(r.get("k7"), Some(7));
    assert_eq!(r.get("missing"), None);
}

#[test]
fn readers_and_writers_mixed() {
    let r = Arc::new(Registry::new());
    r.incr("x", 100);
    let mut hs = Vec::new();
    for _ in 0..4 {
        let r = Arc::clone(&r);
        hs.push(thread::spawn(move || {
            for _ in 0..500 {
                // читатели крутятся параллельно с писателями
                let _ = r.get("x");
                let _ = r.sum();
                r.incr("x", 1);
            }
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
    assert_eq!(r.get("x"), Some(100 + 4 * 500));
}
