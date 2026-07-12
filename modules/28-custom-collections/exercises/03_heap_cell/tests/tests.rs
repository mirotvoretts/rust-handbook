use ex_28_03_heap_cell::*;

#[test]
fn roundtrip() {
    let p = alloc_u64(42);
    unsafe {
        assert_eq!(read_u64(p), 42);
        free_u64(p);
    }
}

#[test]
fn many_cells() {
    // Несколько независимых ячеек: каждая своя память, каждая освобождается.
    let ptrs: Vec<_> = (0..100u64).map(alloc_u64).collect();
    for (i, &p) in ptrs.iter().enumerate() {
        unsafe {
            assert_eq!(read_u64(p), i as u64);
        }
    }
    for p in ptrs {
        unsafe { free_u64(p) }
    }
}

#[test]
fn extreme_value() {
    let p = alloc_u64(u64::MAX);
    unsafe {
        assert_eq!(read_u64(p), u64::MAX);
        free_u64(p);
    }
}
