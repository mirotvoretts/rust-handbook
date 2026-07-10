//! 07 (3x) - построчный протокол поверх TCP. Эталонное решение.

use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;

fn handle(mut stream: TcpStream) {
    // читаем из клона потока построчно, пишем в исходный.
    // try_clone: вторая ручка на то же соединение (не в теории модуля).
    // https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.try_clone
    let reader = BufReader::new(stream.try_clone().unwrap());

    let sum: i64 = reader
        .lines()
        // lines() отдаёт io::Result<String>; берём успешные строки.
        // map_while: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map_while
        .map_while(Result::ok)
        // str::parse: разбор строки в число, ошибки отбрасываем.
        // https://doc.rust-lang.org/std/primitive.str.html#method.parse
        .filter_map(|line| line.trim().parse::<i64>().ok())
        .sum();

    // writeln!: форматированная запись со '\n' в конце.
    // https://doc.rust-lang.org/std/macro.writeln.html
    let _ = writeln!(stream, "{sum}");
}

pub fn spawn_sum_server() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            thread::spawn(move || handle(stream));
        }
    });

    addr
}
