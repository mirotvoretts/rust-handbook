//! 02 (0x) - итерация Receiver, сохранение порядка. Эталонное решение.

use std::sync::mpsc;
use std::thread;

pub fn collect_squares(input: Vec<i64>) -> Vec<i64> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for x in input {
            tx.send(x * x).unwrap();
        }
    });

    // один производитель -> порядок отправки сохраняется при приёме
    rx.into_iter().collect()
}
