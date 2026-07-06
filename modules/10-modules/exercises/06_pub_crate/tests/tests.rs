use ex_10_06_pub_crate::config;

#[test]
fn effective_value() {
    assert_eq!(config::effective(Some(3)), 3);
    assert_eq!(config::effective(None), 7);
}

// Замечание: config::internal_default() отсюда вызвать НЕЛЬЗЯ - она pub(crate),
// то есть видна только внутри крейта упражнения, а этот тест - отдельный крейт.
