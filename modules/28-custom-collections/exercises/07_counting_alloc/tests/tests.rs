use ex_28_07_counting_alloc::*;

// Один тест, чтобы измерения не пересекались с параллельными тестами.
// Проверки односторонние (>=), поэтому фоновые выделения теста им не мешают.
#[test]
fn tracks_heap_usage() {
    let total0 = total_allocated();
    let count0 = alloc_count();

    let mut v: Vec<u8> = Vec::with_capacity(100_000);
    v.extend(std::iter::repeat(7u8).take(100_000));
    std::hint::black_box(&v);

    // Выделили как минимум буфер вектора.
    assert!(
        total_allocated() >= total0 + 100_000,
        "total_allocated должен вырасти хотя бы на размер буфера"
    );
    assert!(
        alloc_count() > count0,
        "alloc_count должен увеличиться хотя бы на одно выделение"
    );

    drop(v);

    // live_bytes: большой буфер должен подняться, а затем освободиться.
    let live0 = live_bytes();
    {
        let big: Vec<u8> = Vec::with_capacity(500_000);
        std::hint::black_box(&big);
        assert!(
            live_bytes() >= live0 + 500_000,
            "live_bytes должен подняться на размер большого буфера"
        );
    }
    assert!(
        live_bytes() < live0 + 500_000,
        "после освобождения live_bytes должен опуститься"
    );
}
