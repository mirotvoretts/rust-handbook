use sol_15_05_marker_trait::{deploy, StableSort};

#[test]
fn approved_deploys() {
    assert_eq!(deploy(&StableSort), "deployed: stable-sort");
}

// deploy(&ExperimentalSort) не компилируется — маркера нет.
// (Проверить «не компилируется» обычным тестом нельзя; поверьте компилятору.)
