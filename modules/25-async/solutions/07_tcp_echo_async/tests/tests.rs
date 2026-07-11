use sol_25_07_tcp_echo_async::spawn_echo_server;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::thread;

fn round_trip(addr: SocketAddr, payload: &[u8]) -> Vec<u8> {
    let mut stream = TcpStream::connect(addr).unwrap();
    stream.write_all(payload).unwrap();
    stream.shutdown(Shutdown::Write).unwrap();
    let mut got = Vec::new();
    stream.read_to_end(&mut got).unwrap();
    got
}

#[test]
fn echoes_payload() {
    let addr = spawn_echo_server();
    assert_eq!(round_trip(addr, b"hello world"), b"hello world");
}

#[test]
fn echoes_empty() {
    let addr = spawn_echo_server();
    assert_eq!(round_trip(addr, b""), b"");
}

#[test]
fn handles_multiple_clients() {
    let addr = spawn_echo_server();
    let mut handles = Vec::new();
    for i in 0..10u8 {
        handles.push(thread::spawn(move || {
            let payload = vec![i; 32];
            assert_eq!(round_trip(addr, &payload), payload);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}
