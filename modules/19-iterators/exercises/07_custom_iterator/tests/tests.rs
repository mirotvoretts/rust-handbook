use ex_19_07_custom_iterator::{sum_of_squares_countdown, Countdown};

#[test]
fn yields_descending() {
    let v: Vec<u32> = Countdown::from(3).collect();
    assert_eq!(v, vec![3, 2, 1]);
}

#[test]
fn empty_countdown() {
    let v: Vec<u32> = Countdown::from(0).collect();
    assert_eq!(v, Vec::<u32>::new());
}

#[test]
fn adapters_come_free() {
    // filter/map/collect работают на своём итераторе без доп. кода
    let evens: Vec<u32> = Countdown::from(6).filter(|x| x % 2 == 0).collect();
    assert_eq!(evens, vec![6, 4, 2]);

    let doubled: Vec<u32> = Countdown::from(3).map(|x| x * 10).collect();
    assert_eq!(doubled, vec![30, 20, 10]);
}

#[test]
fn works_in_for_loop() {
    let mut acc = Vec::new();
    for x in Countdown::from(2) {
        acc.push(x);
    }
    assert_eq!(acc, vec![2, 1]);
}

#[test]
fn sum_of_squares() {
    assert_eq!(sum_of_squares_countdown(3), 14); // 9 + 4 + 1
    assert_eq!(sum_of_squares_countdown(0), 0);
}
