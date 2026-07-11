//! 02 (0x) - .await в цикле: последовательная обработка вектора. Эталонное решение.

use futures::executor::block_on;

async fn transform(x: i64) -> i64 {
    x * 2
}

pub fn process_all(values: Vec<i64>) -> Vec<i64> {
    block_on(async {
        let mut out = Vec::with_capacity(values.len());
        for v in values {
            // последовательный await: следующий элемент начнётся после текущего.
            out.push(transform(v).await);
        }
        out
    })
}
