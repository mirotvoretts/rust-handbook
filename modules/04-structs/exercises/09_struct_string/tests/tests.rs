use ex_04_09_struct_string::User;

#[test]
fn reading() {
    let u = User::new("Аня", 30);
    assert_eq!(u.name(), "Аня");
    assert_eq!(u.age(), 30);
    assert_eq!(u.greet(), "Привет, Аня!");
}

#[test]
fn mutation() {
    let mut u = User::new("Боб", 41);
    u.birthday();
    assert_eq!(u.age(), 42);
    // имя не изменилось
    assert_eq!(u.name(), "Боб");
}
