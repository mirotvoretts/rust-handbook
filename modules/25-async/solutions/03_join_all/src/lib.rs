//! 03 (1x) - конкурентный запуск набора Future через join_all. Эталонное решение.

use futures::executor::block_on;
use futures::future::join_all;

pub fn squares_concurrent(values: Vec<i64>) -> Vec<i64> {
    block_on(async {
        // каждый элемент -> отдельный Future; join_all ведёт их конкурентно
        // и собирает результаты в порядке входа.
        let futs = values.into_iter().map(|x| async move { x * x });
        join_all(futs).await
    })
}
