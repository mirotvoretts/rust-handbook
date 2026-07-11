use sol_25_08_accumulator_server_async::spawn_accumulator_server;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::thread;

fn command(addr: SocketAddr, cmd: &str) -> String {
    let mut stream = TcpStream::connect(addr).unwrap();
    writeln!(stream, "{cmd}").unwrap();
    stream.shutdown(Shutdown::Write).unwrap();
    let mut resp = String::new();
    stream.read_to_string(&mut resp).unwrap();
    resp.trim().to_string()
}

#[test]
fn single_add_and_total() {
    let addr = spawn_accumulator_server();
    assert_eq!(command(addr, "ADD 5"), "5");
    assert_eq!(command(addr, "ADD 10"), "15");
    assert_eq!(command(addr, "TOTAL"), "15");
}

#[test]
fn unknown_command() {
    let addr = spawn_accumulator_server();
    assert_eq!(command(addr, "FOO"), "ERR");
    assert_eq!(command(addr, "TOTAL"), "0");
}

#[test]
fn concurrent_adds_accumulate_correctly() {
    let addr = spawn_accumulator_server();
    let mut handles = Vec::new();
    for i in 1..=20i64 {
        handles.push(thread::spawn(move || {
            let reply = command(addr, &format!("ADD {i}"));
            reply.parse::<i64>().unwrap();
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(command(addr, "TOTAL"), "210");
}
