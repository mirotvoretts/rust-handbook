use sol_12_09_min_core_trait::{Fibs, Samples, Stats};

#[test]
fn samples_stats() {
    let s = Samples(vec![1.0, 2.0, 3.0]);
    assert_eq!(s.count(), 3);
    assert_eq!(s.total(), 6.0);
    assert_eq!(s.mean(), Some(2.0));
}

#[test]
fn empty_mean_is_none() {
    let s = Samples(vec![]);
    assert_eq!(s.count(), 0);
    assert_eq!(s.total(), 0.0);
    assert_eq!(s.mean(), None);
}

#[test]
fn fibs_values() {
    assert_eq!(Fibs(6).values(), vec![1.0, 1.0, 2.0, 3.0, 5.0, 8.0]);
    assert_eq!(Fibs(0).values(), Vec::<f64>::new());
}

#[test]
fn fibs_get_defaults_for_free() {
    let f = Fibs(5); // 1 1 2 3 5
    assert_eq!(f.count(), 5);
    assert_eq!(f.total(), 12.0);
    assert_eq!(f.mean(), Some(2.4));
}
