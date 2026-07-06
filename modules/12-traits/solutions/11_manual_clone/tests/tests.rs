use sol_12_11_manual_clone::TypedId;

// Тип-метка БЕЗ Clone: derive(Clone) на TypedId сделал бы этот тест некомпилируемым.
struct NotCloneTag;

#[test]
fn clone_works_without_t_clone() {
    let id: TypedId<NotCloneTag> = TypedId::new(42);
    let copy = id.clone();
    assert_eq!(copy.id, 42);
}

#[test]
fn copy_works_too() {
    let id: TypedId<NotCloneTag> = TypedId::new(7);
    let a = id; // move? нет — Copy!
    let b = id; // если Copy не реализован, здесь use-after-move
    assert_eq!(a.id + b.id, 14);
}

struct UserTag;
struct OrderTag;

fn take_user_id(id: TypedId<UserTag>) -> u64 {
    id.id
}

#[test]
fn ids_of_different_tags_are_different_types() {
    let user: TypedId<UserTag> = TypedId::new(1);
    let _order: TypedId<OrderTag> = TypedId::new(2);
    assert_eq!(take_user_id(user), 1);
    // take_user_id(_order) — не скомпилировалось бы: разные типы
}
