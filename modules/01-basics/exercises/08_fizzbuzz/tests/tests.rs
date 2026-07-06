use ex_01_08_fizzbuzz::fizzbuzz;

#[test]
fn first_five() {
    assert_eq!(fizzbuzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
}

#[test]
fn length_matches_n() {
    assert_eq!(fizzbuzz(0).len(), 0);
    assert_eq!(fizzbuzz(15).len(), 15);
}

#[test]
fn fizzbuzz_at_fifteen() {
    let v = fizzbuzz(15);
    assert_eq!(v[2], "Fizz"); // 3
    assert_eq!(v[4], "Buzz"); // 5
    assert_eq!(v[14], "FizzBuzz"); // 15
    assert_eq!(v[9], "Buzz"); // 10
}
