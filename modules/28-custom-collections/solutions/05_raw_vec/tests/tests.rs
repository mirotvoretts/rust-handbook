use sol_28_05_raw_vec::*;

#[test]
fn starts_empty() {
    let rv: RawVec<u32> = RawVec::new();
    assert_eq!(rv.capacity(), 0);
}

#[test]
fn growth_schedule() {
    let mut rv: RawVec<u64> = RawVec::new();
    rv.grow();
    assert_eq!(rv.capacity(), 4);
    rv.grow();
    assert_eq!(rv.capacity(), 8);
    rv.grow();
    assert_eq!(rv.capacity(), 16);
}

#[test]
fn buffer_is_usable() {
    let mut rv: RawVec<u32> = RawVec::new();
    rv.grow();
    rv.grow(); // cap >= 8
    assert!(rv.capacity() >= 8);
    let p = rv.ptr().as_ptr();
    unsafe {
        for i in 0..8 {
            p.add(i).write(i as u32 * 10);
        }
        for i in 0..8 {
            assert_eq!(p.add(i).read(), i as u32 * 10);
        }
    }
    // rv роняется тут: буфер освобождается (утечку/двойное освобождение поймал бы miri).
}

#[test]
fn survives_realloc_move() {
    // После realloc данные должны сохраниться (realloc копирует старое содержимое).
    let mut rv: RawVec<u32> = RawVec::new();
    rv.grow(); // cap 4
    let p = rv.ptr().as_ptr();
    unsafe {
        for i in 0..4 {
            p.add(i).write(i as u32 + 1);
        }
    }
    rv.grow(); // cap 8, возможно с переездом блока
    let p = rv.ptr().as_ptr();
    unsafe {
        for i in 0..4 {
            assert_eq!(p.add(i).read(), i as u32 + 1);
        }
    }
}
