use ex_18_06_retry::retry;

#[test]
fn succeeds_on_third_attempt() {
    let mut tries = 0;
    let r = retry(5, || {
        tries += 1;
        if tries < 3 {
            Err("not yet")
        } else {
            Ok(tries)
        }
    });
    assert_eq!(r, Ok(3));
    assert_eq!(tries, 3); // остановились, как только получили Ok
}

#[test]
fn returns_last_error_when_exhausted() {
    let mut calls = 0;
    let r: Result<i32, &str> = retry(4, || {
        calls += 1;
        Err("boom")
    });
    assert_eq!(r, Err("boom"));
    assert_eq!(calls, 4); // ровно n попыток
}

#[test]
fn first_ok_costs_one_call() {
    let mut calls = 0;
    let r: Result<i32, ()> = retry(10, || {
        calls += 1;
        Ok(7)
    });
    assert_eq!(r, Ok(7));
    assert_eq!(calls, 1);
}

#[test]
fn zero_still_tries_once() {
    let mut calls = 0;
    let r: Result<i32, &str> = retry(0, || {
        calls += 1;
        Err("x")
    });
    assert_eq!(r, Err("x"));
    assert_eq!(calls, 1);
}
