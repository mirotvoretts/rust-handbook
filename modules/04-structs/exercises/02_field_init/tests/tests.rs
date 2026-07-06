use ex_04_02_field_init::{grey, rgb, with_red, Color};

#[test]
fn from_components() {
    let c = rgb(10, 20, 30);
    assert_eq!(c, Color { r: 10, g: 20, b: 30 });
}

#[test]
fn greyscale() {
    assert_eq!(grey(128), Color { r: 128, g: 128, b: 128 });
}

#[test]
fn update() {
    let base = rgb(1, 2, 3);
    let red = with_red(base, 255);
    assert_eq!(red, Color { r: 255, g: 2, b: 3 });
    // base - Copy, поэтому всё ещё доступен
    assert_eq!(base, Color { r: 1, g: 2, b: 3 });
}
