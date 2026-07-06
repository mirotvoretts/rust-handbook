use ex_10_08_module_tree::store;

#[test]
fn inventory_count() {
    assert_eq!(store::inventory::count(), 3);
}

#[test]
fn cross_module_total() {
    // 3 (склад) + 2 (ожидающие) == 5
    assert_eq!(store::orders::total_needed(), 5);
}
