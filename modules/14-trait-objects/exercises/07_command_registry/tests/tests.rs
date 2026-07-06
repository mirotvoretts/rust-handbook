use ex_14_07_command_registry::{Accumulator, Command, Echo, Registry};

#[test]
fn commands_standalone() {
    let mut acc = Accumulator { total: 0 };
    assert_eq!(acc.execute(5), "total: 5");
    assert_eq!(acc.execute(3), "total: 8");
    assert_eq!(Echo.execute(42), "echo: 42");
}

#[test]
fn registry_dispatches_by_name() {
    let mut r = Registry::new();
    r.register("acc", Box::new(Accumulator { total: 0 }));
    r.register("echo", Box::new(Echo));

    assert_eq!(r.dispatch("echo", 1), Some(String::from("echo: 1")));
    assert_eq!(r.dispatch("acc", 10), Some(String::from("total: 10")));
    assert_eq!(r.dispatch("acc", 5), Some(String::from("total: 15"))); // состояние живёт
    assert_eq!(r.dispatch("nope", 0), None);
}

// Свой тип из теста - реестр открыт для расширения без правки крейта.
struct Doubler;
impl Command for Doubler {
    fn execute(&mut self, arg: i64) -> String {
        format!("{}", arg * 2)
    }
}

#[test]
fn open_for_extension() {
    let mut r = Registry::new();
    r.register("x2", Box::new(Doubler));
    assert_eq!(r.dispatch("x2", 21), Some(String::from("42")));
}
