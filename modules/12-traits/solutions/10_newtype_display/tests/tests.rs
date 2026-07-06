use sol_12_10_newtype_display::CsvRow;

#[test]
fn displays_joined() {
    let row = CsvRow(vec![String::from("a"), String::from("b"), String::from("c")]);
    assert_eq!(row.to_string(), "a, b, c"); // to_string приходит бесплатно из Display
}

#[test]
fn empty_row() {
    assert_eq!(CsvRow::new().to_string(), "");
}

#[test]
fn push_then_display() {
    let mut row = CsvRow::new();
    row.push("id");
    row.push("name");
    assert_eq!(format!("[{row}]"), "[id, name]");
}

#[test]
fn inner_vec_reachable() {
    let mut row = CsvRow::new();
    row.push("x");
    assert_eq!(row.0.len(), 1); // цена newtype: доступ через .0
}
