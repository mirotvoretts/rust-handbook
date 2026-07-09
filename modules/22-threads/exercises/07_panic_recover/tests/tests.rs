use ex_22_07_panic_recover::run_jobs;

#[test]
fn counts_ok_and_panicked() {
    // 3 неотрицательных, 2 отрицательных
    assert_eq!(run_jobs(vec![1, -2, 3, -4, 5]), (3, 2));
}

#[test]
fn all_ok() {
    assert_eq!(run_jobs(vec![0, 7, 42]), (3, 0));
}

#[test]
fn all_panic() {
    assert_eq!(run_jobs(vec![-1, -1]), (0, 2));
}

#[test]
fn empty() {
    assert_eq!(run_jobs(vec![]), (0, 0));
}

#[test]
fn main_thread_survives() {
    // если бы паника роняла процесс, до сюда бы не дошли
    let _ = run_jobs(vec![-9]);
    assert!(true);
}
