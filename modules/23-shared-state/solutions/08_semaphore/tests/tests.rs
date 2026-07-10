use sol_23_08_semaphore::Semaphore;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn respects_max_concurrency() {
    let k = 3;
    let sem = Arc::new(Semaphore::new(k));
    let current = Arc::new(AtomicUsize::new(0));
    let max_seen = Arc::new(AtomicUsize::new(0));
    let done = Arc::new(AtomicUsize::new(0));

    let mut hs = Vec::new();
    for _ in 0..24 {
        let sem = Arc::clone(&sem);
        let cur = Arc::clone(&current);
        let mx = Arc::clone(&max_seen);
        let dn = Arc::clone(&done);
        hs.push(thread::spawn(move || {
            sem.acquire();
            let now = cur.fetch_add(1, Ordering::SeqCst) + 1;
            mx.fetch_max(now, Ordering::SeqCst);
            thread::sleep(Duration::from_millis(2));
            cur.fetch_sub(1, Ordering::SeqCst);
            dn.fetch_add(1, Ordering::SeqCst);
            sem.release();
        }));
    }
    for h in hs {
        h.join().unwrap();
    }

    assert!(max_seen.load(Ordering::SeqCst) <= k);
    assert_eq!(done.load(Ordering::SeqCst), 24);
    assert_eq!(current.load(Ordering::SeqCst), 0);
}

#[test]
fn permit_one_is_mutex() {
    let sem = Arc::new(Semaphore::new(1));
    let counter = Arc::new(AtomicUsize::new(0));

    let mut hs = Vec::new();
    for _ in 0..8 {
        let sem = Arc::clone(&sem);
        let c = Arc::clone(&counter);
        hs.push(thread::spawn(move || {
            for _ in 0..1000 {
                sem.acquire();
                let v = c.load(Ordering::Relaxed);
                c.store(v + 1, Ordering::Relaxed);
                sem.release();
            }
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
    assert_eq!(counter.load(Ordering::Relaxed), 8000);
}
