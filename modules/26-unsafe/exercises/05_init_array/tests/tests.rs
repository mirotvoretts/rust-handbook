use ex_26_05_init_array::labels;

#[test]
fn contents() {
    let got = labels();
    assert_eq!(
        got,
        [
            "item-0".to_string(),
            "item-1".to_string(),
            "item-2".to_string(),
            "item-3".to_string(),
            "item-4".to_string(),
        ]
    );
}

#[test]
fn length_is_five() {
    assert_eq!(labels().len(), 5);
}

#[test]
fn each_owned_string_usable() {
    // элементы должны быть настоящими владеющими String, а не мусором:
    let mut arr = labels();
    arr[0].push_str("-x");
    assert_eq!(arr[0], "item-0-x");
    assert_eq!(arr[4], "item-4");
}
