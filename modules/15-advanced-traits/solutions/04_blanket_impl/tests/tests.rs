use sol_15_04_blanket_impl::{Loggable, Temperature};

#[test]
fn std_types_are_loggable() {
    assert_eq!(5.log_line(), "[LOG] 5");
    assert_eq!("hi".log_line(), "[LOG] hi");
    assert_eq!(true.log_line(), "[LOG] true");
}

#[test]
fn own_type_via_display() {
    assert_eq!(Temperature(21.53).log_line(), "[LOG] 21.5C");
}
