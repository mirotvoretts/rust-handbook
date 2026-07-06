use sol_06_12_move_only_type::FileHandle;

#[test]
fn open_access_close() {
    let h = FileHandle::open(3, "/tmp/data.txt");
    assert_eq!(h.descriptor(), 3);
    assert_eq!(h.path(), "/tmp/data.txt");
    let path = h.close();
    assert_eq!(path, "/tmp/data.txt");
    // после close переменная `h` moved-from — обратиться к ней уже нельзя (проверено компилятором)
}

#[test]
fn handle_moves_between_owners() {
    let a = FileHandle::open(7, "log");
    let b = a; // move: a теперь moved-from
    assert_eq!(b.descriptor(), 7);
    assert_eq!(b.close(), "log");
}
