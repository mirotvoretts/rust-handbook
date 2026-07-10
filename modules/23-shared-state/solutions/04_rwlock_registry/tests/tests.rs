use sol_23_04_rwlock_registry::Registry;
use std::sync::Arc;
use std::thread;

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
fn distinct_keys() {
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
    assert_eq!(r.get("missing"), None);
}
