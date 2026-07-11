use ex_25_04_ready_future::Ready;
use futures::executor::block_on;

#[test]
fn yields_value() {
    assert_eq!(block_on(Ready(Some(42))), 42);
}

#[test]
fn awaitable_in_block() {
    let out = block_on(async { Ready(Some(7)).await + 1 });
    assert_eq!(out, 8);
}

#[test]
fn composes_with_other_futures() {
    let out = block_on(async {
        let a = Ready(Some(10)).await;
        let b = Ready(Some(20)).await;
        a + b
    });
    assert_eq!(out, 30);
}
