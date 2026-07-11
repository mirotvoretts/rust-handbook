//! 08 (3x) - многоклиентный async-сервер с общим состоянием.
//!
//! Асинхронный аналог упражнения M24-08. Сервер держит общий целочисленный
//! аккумулятор (изначально 0) и обрабатывает соединения конкурентно на async-задачах
//! tokio, разделяя аккумулятор между ними. Протокол - одна строка-команда на
//! соединение:
//!   "ADD n\n"  - прибавить n к аккумулятору, в ответ прислать новое значение и '\n';
//!   "TOTAL\n"  - прислать текущее значение аккумулятора и '\n', ничего не меняя;
//!   любая другая строка - ответ "ERR\n".
//!
//! Требования к запуску те же, что в 07: bind на "127.0.0.1:0", адрес наружу, accept
//! -цикл и обработчики - на рантайме tokio; аккумулятор общий для всех соединений.
//!
//! Внимание к синхронизации в async. Обычный std::sync::Mutex использовать можно, но
//! его guard (`MutexGuard`) НЕ Send, поэтому держать его через точку `.await` нельзя
//! (задача с таким guard'ом не соберётся в tokio::spawn или рискует дедлоком). Значит:
//! возьми значение под замком и отпусти guard ДО любого await (например, до записи
//! ответа в сокет), сузив область блокировки. Альтернатива - асинхронный
//! tokio::sync::Mutex, чей guard держать через await можно (см. ссылку ниже).
//!
//! Что понадобится (за пределами теории):
//! - учебник tokio: https://tokio.rs/tokio/tutorial
//! - разделить сокет на чтение и запись: TcpStream::into_split
//!   https://docs.rs/tokio/latest/tokio/net/struct.TcpStream.html#method.into_split
//! - построчное async-чтение: tokio::io::BufReader + AsyncBufReadExt::read_line
//!   https://docs.rs/tokio/latest/tokio/io/trait.AsyncBufReadExt.html#method.read_line
//! - отделить префикс команды: str::strip_prefix
//!   https://doc.rust-lang.org/std/primitive.str.html#method.strip_prefix
//! - разделяемое состояние - Arc<Mutex<..>> из M23; про async-замок:
//!   https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html

use std::net::SocketAddr;

/// Запусти async-сервер-аккумулятор в фоне и верни его адрес.
pub fn spawn_accumulator_server() -> SocketAddr {
    todo!()
}
