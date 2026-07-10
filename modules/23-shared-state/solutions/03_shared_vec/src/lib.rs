//! 03 (1x) - Arc<Mutex<Vec<T>>>. Эталонное решение.

use std::sync::{Arc, Mutex};
use std::thread;

pub fn parallel_squares(input: Vec<i64>) -> Vec<i64> {
    let out = Arc::new(Mutex::new(Vec::with_capacity(input.len())));
    let mut handles = Vec::with_capacity(input.len());

    for x in input {
        let o = Arc::clone(&out);
        handles.push(thread::spawn(move || {
            let sq = x * x;
            o.lock().unwrap().push(sq);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let mut result = Arc::try_unwrap(out).unwrap().into_inner().unwrap();
    result.sort();
    result
}
