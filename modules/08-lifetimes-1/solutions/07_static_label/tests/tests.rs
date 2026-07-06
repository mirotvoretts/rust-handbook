use sol_08_07_static_label::size_name;

#[test]
fn labels() {
    assert_eq!(size_name(0), "small");
    assert_eq!(size_name(9), "small");
    assert_eq!(size_name(10), "medium");
    assert_eq!(size_name(99), "medium");
    assert_eq!(size_name(100), "large");
    assert_eq!(size_name(1000), "large");
}
