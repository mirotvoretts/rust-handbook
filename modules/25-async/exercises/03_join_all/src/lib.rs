//! 03 (1x) - конкурентный запуск набора Future через join_all.
//!
//! Реализуй `squares_concurrent(values)`, который для каждого значения строит Future,
//! вычисляющий его квадрат, и запускает весь набор Future *конкурентно* через
//! join_all, а не по одному в цикле (сравни с упражнением 02). Результаты join_all
//! возвращает в исходном порядке входа. Итоговый Future запусти через block_on.
//!
//! squares_concurrent(vec![1, 2, 3, 4]) == vec![1, 4, 9, 16].
//!
//! Конструкции за пределами теории:
//! - конкурентный запуск коллекции Future: futures::future::join_all
//!   (см. https://docs.rs/futures/latest/futures/future/fn.join_all.html);
//! - запуск Future из синхронного кода: futures::executor::block_on
//!   (см. https://docs.rs/futures/latest/futures/executor/fn.block_on.html).

/// Конкурентно возведи каждое значение в квадрат и верни результаты в исходном
/// порядке.
pub fn squares_concurrent(values: Vec<i64>) -> Vec<i64> {
    todo!()
}
