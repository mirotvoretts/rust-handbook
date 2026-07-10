//! 06 (2x) - TCP эхо-сервер. Эталонное решение.

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};
use std::thread;

pub fn spawn_echo_server() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = match stream {
                Ok(s) => s,
                Err(_) => continue,
            };
            thread::spawn(move || {
                let mut buf = Vec::new();
                if stream.read_to_end(&mut buf).is_ok() {
                    let _ = stream.write_all(&buf);
                }
            });
        }
    });

    addr
}
