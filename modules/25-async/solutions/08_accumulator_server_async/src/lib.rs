//! 08 (3x) - многоклиентный async-сервер с общим состоянием. Эталонное решение.

use std::net::SocketAddr;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

async fn handle(stream: TcpStream, total: Arc<Mutex<i64>>) {
    // разделяем сокет: из read-половины читаем строку, в write-половину пишем ответ.
    let (read_half, mut write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half);

    let mut line = String::new();
    if reader.read_line(&mut line).await.is_err() {
        return;
    }
    let line = line.trim();

    // std::Mutex: guard берём и отпускаем ДО await записи (в отдельном блоке),
    // иначе не-Send guard пережил бы точку .await.
    let response = if let Some(rest) = line.strip_prefix("ADD ") {
        let n: i64 = rest.trim().parse().unwrap_or(0);
        let new = {
            let mut acc = total.lock().unwrap();
            *acc += n;
            *acc
        };
        format!("{new}\n")
    } else if line == "TOTAL" {
        let value = *total.lock().unwrap();
        format!("{value}\n")
    } else {
        "ERR\n".to_string()
    };

    let _ = write_half.write_all(response.as_bytes()).await;
}

pub fn spawn_accumulator_server() -> SocketAddr {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            tx.send(listener.local_addr().unwrap()).unwrap();

            let total = Arc::new(Mutex::new(0i64));
            loop {
                let (socket, _) = listener.accept().await.unwrap();
                let total = Arc::clone(&total);
                tokio::spawn(handle(socket, total));
            }
        });
    });

    rx.recv().unwrap()
}
