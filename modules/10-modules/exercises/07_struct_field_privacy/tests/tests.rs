use ex_10_07_struct_field_privacy::bank::Account;

#[test]
fn encapsulated() {
    let mut acc = Account::new(100);
    assert_eq!(acc.balance(), 100);
    acc.deposit(50);
    assert_eq!(acc.balance(), 150);
    // acc.balance — это поле — недоступно снаружи модуля (только метод .balance())
}
