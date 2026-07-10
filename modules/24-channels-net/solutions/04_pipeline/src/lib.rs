//! 04 (1x) - двухстадийный конвейер. Эталонное решение.

use std::sync::mpsc;
use std::thread;

pub fn pipeline<F>(input: Vec<i64>, f: F) -> Vec<i64>
where
    F: Fn(i64) -> i64 + Send + 'static,
{
    let (tx1, rx1) = mpsc::channel(); // источник -> воркер
    let (tx2, rx2) = mpsc::channel(); // воркер -> сток

    // источник
    thread::spawn(move || {
        for x in input {
            tx1.send(x).unwrap();
        }
    });

    // воркер: читает rx1, применяет f, пишет в tx2
    thread::spawn(move || {
        for x in rx1 {
            tx2.send(f(x)).unwrap();
        }
    });

    // сток (текущий поток) - порядок сохраняется, производитель на каждом канале один
    rx2.into_iter().collect()
}
