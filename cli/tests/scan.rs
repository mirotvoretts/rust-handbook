//! Интеграционные тесты сканирования курса и логики выбора упражнений на наборе
//! фикстур-модулей в `tests/fixtures/modules/`.

use std::path::PathBuf;

use rh::course::{Course, Tier};
use rh::progress::{Progress, Status};

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/modules")
}

fn course() -> Course {
    Course::scan(&fixtures()).expect("scan fixtures")
}

#[test]
fn scans_modules_in_number_order() {
    let c = course();
    let nums: Vec<u32> = c.modules.iter().map(|m| m.number).collect();
    assert_eq!(nums, vec![3, 7]);
}

#[test]
fn reads_module_titles() {
    let c = course();
    assert!(c.modules[0].title.contains("Ранний"));
    assert!(c.modules[1].title.contains("Демо-модуль"));
}

#[test]
fn scans_exercises_in_number_order_with_tiers() {
    let c = course();
    let m7 = c.modules.iter().find(|m| m.number == 7).unwrap();
    let names: Vec<&str> = m7.exercises.iter().map(|e| e.ex_pkg.as_str()).collect();
    assert_eq!(names, vec!["ex-07-01-alpha", "ex-07-11-beta"]);
    assert_eq!(m7.exercises[0].tier, Tier::Syntax);
    assert_eq!(m7.exercises[1].tier, Tier::Basic);
}

#[test]
fn total_across_modules() {
    assert_eq!(course().total_exercises(), 3);
}

#[test]
fn pairs_solution_package_name() {
    let c = course();
    let ex = c.find("ex-07-11-beta").unwrap();
    assert_eq!(ex.sol_pkg.as_deref(), Some("sol-07-11-beta"));
}

#[test]
fn next_unsolved_follows_course_order() {
    let c = course();
    let mut p = Progress::default();

    // Ничего не решено -> первое по порядку (модуль 3).
    let next = c
        .exercises()
        .find(|e| !p.is_passed(&e.ex_pkg))
        .map(|e| e.ex_pkg.clone());
    assert_eq!(next.as_deref(), Some("ex-03-21-solo"));

    // Решили первое -> следующее из модуля 7.
    p.record("ex-03-21-solo", Status::Pass, "h".into());
    let next = c
        .exercises()
        .find(|e| !p.is_passed(&e.ex_pkg))
        .map(|e| e.ex_pkg.clone());
    assert_eq!(next.as_deref(), Some("ex-07-01-alpha"));
}
