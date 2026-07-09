//! 07 (3x) - изоляция паники. Эталонное решение.

use std::thread;

/// По потоку на задачу; поток паникует при отрицательном числе. Возвращаем
/// (успешные, упавшие).
pub fn run_jobs(jobs: Vec<i32>) -> (usize, usize) {
    let handles: Vec<_> = jobs
        .into_iter()
        .map(|job| {
            thread::spawn(move || {
                if job < 0 {
                    panic!("job {job} failed");
                }
            })
        })
        .collect();

    let mut ok = 0;
    let mut panicked = 0;
    for h in handles {
        if h.join().is_ok() {
            ok += 1;
        } else {
            panicked += 1;
        }
    }
    (ok, panicked)
}
