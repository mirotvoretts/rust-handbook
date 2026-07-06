use sol_15_03_assoc_vs_param::{Convert, Kilometers, MetersOut, Miles};

#[test]
fn two_impls_coexist_on_one_type() {
    let km = Kilometers(2.0);
    let mi: Miles = km.convert();          // аннотация выбирает impl
    let m: MetersOut = km.convert();
    assert!((mi.0 - 1.242742).abs() < 1e-9);
    assert_eq!(m, MetersOut(2000.0));
}

#[test]
fn turbofish_also_works() {
    let km = Kilometers(1.0);
    assert_eq!(Convert::<MetersOut>::convert(&km), MetersOut(1000.0));
}
