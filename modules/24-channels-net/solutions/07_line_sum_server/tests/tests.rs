use sol_24_07_line_sum_server::spawn_sum_server;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::thread;

fn query(addr: SocketAddr, body: &str) -> String {
    let mut stream = TcpStream::connect(addr).unwrap();
    stream.write_all(body.as_bytes()).unwrap();
    stream.shutdown(Shutdown::Write).unwrap();
    let mut resp = String::new();
    stream.read_to_string(&mut resp).unwrap();
    resp.trim().to_string()
}

#[test]
fn sums_lines() {
    let addr = spawn_sum_server();
    assert_eq!(query(addr, "1\n2\n3\n40\n"), "46");
}

#[test]
fn ignores_garbage_and_blanks() {
    let addr = spawn_sum_server();
    assert_eq!(query(addr, "10\n\nhello\n5\n\n"), "15");
}

#[test]
fn empty_input() {
    let addr = spawn_sum_server();
    assert_eq!(query(addr, ""), "0");
}

#[test]
fn concurrent_clients() {
    let addr = spawn_sum_server();
    let mut handles = Vec::new();
    for i in 1..=8i64 {
        handles.push(thread::spawn(move || {
            let body = format!("{i}\n{i}\n{i}\n");
            assert_eq!(query(addr, &body), (3 * i).to_string());
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}
