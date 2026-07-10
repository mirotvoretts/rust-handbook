use ex_23_08_semaphore::Semaphore;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Инвариант: в критической зоне между acquire и release никогда не бывает больше
/// `k` потоков одновременно.
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
            thread::sleep(Duration::from_millis(2)); // держим зону, провоцируя перекрытие
            cur.fetch_sub(1, Ordering::SeqCst);
            dn.fetch_add(1, Ordering::SeqCst);
            sem.release();
        }));
    }
    for h in hs {
        h.join().unwrap();
    }

    assert!(
        max_seen.load(Ordering::SeqCst) <= k,
        "в зоне оказалось больше k потоков одновременно"
    );
    assert!(max_seen.load(Ordering::SeqCst) >= 1);
    assert_eq!(done.load(Ordering::SeqCst), 24);
    assert_eq!(current.load(Ordering::SeqCst), 0);
}

/// permits == 1: семафор работает как мьютекс, инкременты не теряются.
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
                // намеренно не атомарный read-modify-write: корректен только
                // потому, что семафор обеспечивает взаимное исключение
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
