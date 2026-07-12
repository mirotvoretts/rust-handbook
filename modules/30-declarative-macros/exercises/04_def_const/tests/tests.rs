use ex_30_04_def_const::def_const;

def_const!(LIMIT: u32 = 100);
def_const!(NAME: &str = "rust");
def_const!(FACTOR: i64 = 6 * 7);

#[test]
fn generated_consts() {
    assert_eq!(LIMIT, 100);
    assert_eq!(NAME, "rust");
    assert_eq!(FACTOR, 42);
}
