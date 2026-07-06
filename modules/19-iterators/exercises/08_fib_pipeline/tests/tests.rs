use ex_19_08_fib_pipeline::{first_fibs, sum_even_fibs_below, Bag, Fib};

#[test]
fn fib_sequence_manual() {
    let mut f = Fib::new();
    assert_eq!(f.next(), Some(0));
    assert_eq!(f.next(), Some(1));
    assert_eq!(f.next(), Some(1));
    assert_eq!(f.next(), Some(2));
    assert_eq!(f.next(), Some(3));
    assert_eq!(f.next(), Some(5));
}

#[test]
fn take_tames_infinite() {
    assert_eq!(first_fibs(7), vec![0, 1, 1, 2, 3, 5, 8]);
    assert_eq!(first_fibs(0), Vec::<u64>::new());
    assert_eq!(first_fibs(1), vec![0]);
}

#[test]
fn even_fibs_pipeline() {
    // чётные фибоначчи < 100: 0, 2, 8, 34 -> сумма 44
    assert_eq!(sum_even_fibs_below(100), 44);
    // Project Euler #2: чётные < 4_000_000 -> 4613732
    assert_eq!(sum_even_fibs_below(4_000_000), 4_613_732);
}

#[test]
fn bag_into_iter_in_for() {
    let bag = Bag::new(vec![10, 20, 30]);
    let mut total = 0;
    for x in bag {
        total += x;
    }
    assert_eq!(total, 60);
}

#[test]
fn bag_into_iter_with_adapters() {
    let bag = Bag::new(vec![1, 2, 3, 4]);
    let doubled: Vec<i32> = bag.into_iter().map(|x| x * 2).collect();
    assert_eq!(doubled, vec![2, 4, 6, 8]);
}
