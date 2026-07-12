use sol_32_04_door_typestate::Door;

#[test]
fn open_and_walk() {
    let door = Door::new();
    let door = door.open();
    assert_eq!(door.walk_through(), "прошли");
    let _closed = door.close();
}

#[test]
fn lock_cycle() {
    let door = Door::new().lock();
    let _closed = door.unlock();
}

// Не компилируется - инварианты состояний:
// Door::new().walk_through();
// Door::new().lock().open();
