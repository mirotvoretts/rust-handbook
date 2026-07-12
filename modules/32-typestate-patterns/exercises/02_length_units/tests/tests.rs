use ex_32_02_length_units::{Feet, Length, Meters};

#[test]
fn adds_same_unit() {
    let a: Length<Meters> = Length::new(1.0);
    let b: Length<Meters> = Length::new(2.5);
    assert_eq!(a.add(&b).value(), 3.5);
}

#[test]
fn converts_both_ways() {
    let m: Length<Meters> = Length::new(1.0);
    let f: Length<Feet> = m.to_feet();
    assert!((f.value() - 3.280_839_895).abs() < 1e-6);
    let back: Length<Meters> = f.to_meters();
    assert!((back.value() - 1.0).abs() < 1e-9);
}

// Не компилируется (разные единицы) - иллюстрация:
// let m: Length<Meters> = Length::new(1.0);
// let f: Length<Feet> = Length::new(1.0);
// m.add(&f);
