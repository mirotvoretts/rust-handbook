use ex_31_01_make_answer::make_answer;

make_answer!();

#[test]
fn produces_answer() {
    assert_eq!(answer(), 42);
}
