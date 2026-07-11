//! 01 (0x) - async fn и .await: цепочка асинхронных стадий. Эталонное решение.

use futures::executor::block_on;

async fn stage_a(x: i64) -> i64 {
    x + 1
}

async fn stage_b(x: i64) -> i64 {
    x * 2
}

pub fn run_chain(x: i64) -> i64 {
    // async-блок даёт Future; block_on крутит его до Ready и возвращает результат.
    block_on(async {
        let a = stage_a(x).await;
        stage_b(a).await
    })
}
