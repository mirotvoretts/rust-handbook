//! 08 (3x) - многоклиентный сервер с общим состоянием. Эталонное решение.

use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle(mut stream: TcpStream, total: Arc<Mutex<i64>>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut line = String::new();
    // read_line читает до '\n' или EOF (не в теории модуля):
    // https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line
    if reader.read_line(&mut line).is_err() {
        return;
    }
    let line = line.trim();

    // strip_prefix отделяет команду от аргумента (не в теории модуля):
    // https://doc.rust-lang.org/std/primitive.str.html#method.strip_prefix
    if let Some(rest) = line.strip_prefix("ADD ") {
        let n: i64 = rest.trim().parse().unwrap_or(0);
        let mut acc = total.lock().unwrap();
        *acc += n;
        let _ = writeln!(stream, "{}", *acc);
    } else if line == "TOTAL" {
        let acc = total.lock().unwrap();
        let _ = writeln!(stream, "{}", *acc);
    } else {
        let _ = writeln!(stream, "ERR");
    }
}

pub fn spawn_accumulator_server() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let total = Arc::new(Mutex::new(0i64));

    thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let total = Arc::clone(&total);
            thread::spawn(move || handle(stream, total));
        }
    });

    addr
}
