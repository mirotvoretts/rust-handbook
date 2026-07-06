use ex_16_09_drop_panic_guard::Transaction;

#[test]
fn commit_disarms_drop() {
    let t = Transaction::begin();
    assert!(t.commit()); // если Drop сработает — паника провалит тест
}

#[test]
fn rollback_disarms_drop() {
    let t = Transaction::begin();
    assert!(t.rollback());
}

#[test]
#[should_panic(expected = "транзакция")]
fn forgotten_transaction_panics() {
    let _t = Transaction::begin();
    // конец области видимости -> Drop -> паника
}

#[test]
#[should_panic(expected = "другая беда")]
fn no_double_panic_during_unwinding() {
    let _t = Transaction::begin();
    // Паника ниже начинает разматывание; Drop _t обязан промолчать,
    // иначе paniка-в-панике превратится в abort и тест упадёт весь процесс.
    panic!("другая беда");
}
