use ex_32_05_traffic_light::Light;

#[test]
fn cycle() {
    let l = Light::new();
    assert_eq!(l.color(), "red");
    let l = l.next();
    assert_eq!(l.color(), "green");
    let l = l.next();
    assert_eq!(l.color(), "yellow");
    let l = l.next();
    assert_eq!(l.color(), "red");
}
