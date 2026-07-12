use ex_32_01_typed_id::{user_name, Id, Post, User};

#[test]
fn ids_carry_value() {
    let u: Id<User> = Id::new(7);
    assert_eq!(u.value(), 7);
    assert_eq!(user_name(u), "user #7");
}

#[test]
fn same_type_ids_compare() {
    let a: Id<Post> = Id::new(1);
    let b: Id<Post> = Id::new(1);
    assert_eq!(a, b);
    assert_eq!(format!("{a:?}"), "Id(1)");
}

// Не компилируется (разные типы) - иллюстрация:
// let p: Id<Post> = Id::new(1);
// user_name(p);
