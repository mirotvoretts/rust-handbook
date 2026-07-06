use ex_12_01_define_trait::{Cat, Dog, Say};

#[test]
fn cat_meows() {
    assert_eq!(Cat.say(), "meow!");
}

#[test]
fn dog_woofs() {
    assert_eq!(Dog.say(), "woof!");
}

fn speak(x: &dyn Say) -> String {
    x.say()
}

#[test]
fn callable_through_trait() {
    assert_eq!(speak(&Cat), "meow!");
    assert_eq!(speak(&Dog), "woof!");
}
