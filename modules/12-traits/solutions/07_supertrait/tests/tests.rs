use sol_12_07_supertrait::{Animal, Dog, Pet};

fn dog() -> Dog {
    Dog { nickname: String::from("Шарик"), owner_name: String::from("Дядя Фёдор") }
}

#[test]
fn both_impls_exist() {
    let d = dog();
    assert_eq!(d.name(), "Шарик");
    assert_eq!(d.owner(), "Дядя Фёдор");
}

#[test]
fn default_tag_combines_both_traits() {
    assert_eq!(dog().tag(), "Шарик (хозяин: Дядя Фёдор)");
}

// Функция с границей Pet автоматически получает и методы Animal.
fn describe<P: Pet>(p: &P) -> String {
    format!("{}/{}", p.name(), p.owner())
}

#[test]
fn pet_bound_implies_animal() {
    assert_eq!(describe(&dog()), "Шарик/Дядя Фёдор");
}
