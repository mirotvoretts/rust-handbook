use sol_12_05_where_clause::{cheapest_name, receipt, Book, Named, Priced};

fn book(t: &str, k: u32) -> Book {
    Book { title: t.to_string(), kopecks: k }
}

#[test]
fn cheapest() {
    let shelf = [book("Дорогая", 500), book("Дешёвая", 100), book("Средняя", 300)];
    assert_eq!(cheapest_name(&shelf), Some(String::from("Дешёвая")));
}

#[test]
fn cheapest_empty_and_tie() {
    let empty: [Book; 0] = [];
    assert_eq!(cheapest_name(&empty), None);
    let tie = [book("Первая", 100), book("Вторая", 100)];
    assert_eq!(cheapest_name(&tie), Some(String::from("Первая")));
}

struct Sticker;
impl Priced for Sticker {
    fn price(&self) -> u32 { 50 }
}
impl Named for Sticker {
    fn name(&self) -> String { String::from("наклейка") }
}

#[test]
fn receipt_mixes_types() {
    let b = book("Книга", 1000);
    let (total, names) = receipt(&b, &Sticker);
    assert_eq!(total, 1050);
    assert_eq!(names, vec!["Книга", "наклейка"]);
}
