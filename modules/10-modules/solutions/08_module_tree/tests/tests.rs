use sol_10_08_module_tree::store;

#[test]
fn inventory_count() {
    assert_eq!(store::inventory::count(), 3);
}

#[test]
fn cross_module_total() {
    assert_eq!(store::orders::total_needed(), 5);
}
