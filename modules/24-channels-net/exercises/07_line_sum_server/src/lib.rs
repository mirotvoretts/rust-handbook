//! 07 (3x) - построчный протокол поверх TCP.
//!
//! Сервер читает от клиента строки (разделитель - '\n'), в каждой строке - целое
//! число. Дочитав до конца (клиент закрывает запись через shutdown(Write)), сервер
//! отправляет обратно одну строку с суммой всех чисел и '\n' в конце. Строки,
//! которые не разбираются в число (пустые, мусор), игнорируются.
//!
//! Требования те же, что в 06: bind на "127.0.0.1:0", адрес через local_addr,
//! слушающий цикл и обработка соединений - в фоновых потоках.
//!
//! Подсказки по конструкциям:
//! - построчное чтение: заверни поток в BufReader и вызови .lines()
//!   (см. https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines);
//! - читать и писать один и тот же сокет: TcpStream::try_clone даёт вторую половину
//!   на то же соединение (см. https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.try_clone);
//! - разбор строки в число: str::parse
//!   (см. https://doc.rust-lang.org/std/primitive.str.html#method.parse);
//! - запись форматированной строки в поток: макрос writeln!
//!   (см. https://doc.rust-lang.org/std/macro.writeln.html).

use std::net::SocketAddr;

/// Запусти сервер-сумматор строк в фоне и верни его адрес.
pub fn spawn_sum_server() -> SocketAddr {
    todo!()
}
