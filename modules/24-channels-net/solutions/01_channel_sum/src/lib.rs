//! 01 (0x) - mpsc::channel. Эталонное решение.

use std::sync::mpsc;
use std::thread;

pub fn sum_over_channel(values: Vec<i64>) -> i64 {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for v in values {
            tx.send(v).unwrap();
        }
        // tx выходит из области и дропается -> канал закрывается,
        // цикл получателя ниже завершится сам
    });

    let mut sum = 0;
    for v in rx {
        sum += v;
    }
    sum
}
