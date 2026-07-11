use ex_25_05_yield_n::yield_n;
use futures::executor::block_on;

#[test]
fn ready_immediately() {
    assert_eq!(block_on(yield_n(0)), 1);
}

#[test]
fn yields_three_times() {
    assert_eq!(block_on(yield_n(3)), 4);
}

#[test]
fn yields_many_times() {
    assert_eq!(block_on(yield_n(50)), 51);
}

#[test]
fn awaitable_in_block() {
    let out = block_on(async { yield_n(2).await + yield_n(1).await });
    assert_eq!(out, 3 + 2);
}
