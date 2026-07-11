//! 07 (3x) - асинхронный TCP эхо-сервер на tokio. Эталонное решение.

use std::net::SocketAddr;
use std::sync::mpsc;
use std::thread;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub fn spawn_echo_server() -> SocketAddr {
    // рантайм tokio живёт на отдельном потоке; адрес возвращаем через канал.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            tx.send(listener.local_addr().unwrap()).unwrap();

            loop {
                let (mut socket, _) = listener.accept().await.unwrap();
                // каждое соединение - отдельная async-задача, идут конкурентно.
                tokio::spawn(async move {
                    let mut buf = Vec::new();
                    if socket.read_to_end(&mut buf).await.is_ok() {
                        let _ = socket.write_all(&buf).await;
                    }
                });
            }
        });
    });

    rx.recv().unwrap()
}
