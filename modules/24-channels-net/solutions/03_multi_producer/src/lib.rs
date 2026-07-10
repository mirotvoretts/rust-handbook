//! 03 (1x) - клонирование Sender. Эталонное решение.

use std::sync::mpsc;
use std::thread;

pub fn parallel_sum(chunks: Vec<Vec<i64>>) -> i64 {
    let (tx, rx) = mpsc::channel();

    for chunk in chunks {
        let tx = tx.clone(); // отдельный отправитель на поток
        thread::spawn(move || {
            let partial: i64 = chunk.iter().sum();
            tx.send(partial).unwrap();
        });
    }
    drop(tx); // исходный отправитель больше не нужен: закрываем канал

    rx.into_iter().sum()
}
