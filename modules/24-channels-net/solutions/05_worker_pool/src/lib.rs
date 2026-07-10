//! 05 (2x) - пул воркеров на Arc<Mutex<Receiver>>. Эталонное решение.
//!
//! Тот же узор, что в главе про многопоточность The Rust Book (пул потоков веб-
//! сервера): общий приёмник задач под Arc<Mutex<Receiver>>, воркеры берут работу по
//! одной. См. https://doc.rust-lang.org/book/ch21-02-multithreaded.html

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn pool_sum_of_squares(jobs: Vec<u64>, workers: usize) -> u64 {
    let workers = workers.max(1);

    let (job_tx, job_rx) = mpsc::channel::<u64>();
    let (res_tx, res_rx) = mpsc::channel::<u64>();
    let job_rx = Arc::new(Mutex::new(job_rx));

    let mut handles = Vec::with_capacity(workers);
    for _ in 0..workers {
        let job_rx = Arc::clone(&job_rx);
        let res_tx = res_tx.clone();
        handles.push(thread::spawn(move || loop {
            // берём одну задачу под замком и сразу отпускаем замок:
            // guard - временное значение, дропается в конце этого let-выражения
            let job = job_rx.lock().unwrap().recv();
            match job {
                Ok(n) => res_tx.send(n * n).unwrap(),
                Err(_) => break, // канал задач закрыт -> задач больше нет
            }
        }));
    }

    for j in jobs {
        job_tx.send(j).unwrap();
    }
    drop(job_tx); // закрываем канал задач: воркеры получат Err и выйдут
    drop(res_tx); // отпускаем исходный отправитель результатов

    let total: u64 = res_rx.into_iter().sum();
    for h in handles {
        h.join().unwrap();
    }
    total
}
