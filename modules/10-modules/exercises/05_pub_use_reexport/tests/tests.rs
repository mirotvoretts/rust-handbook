// compute доступна на верхнем уровне крейта благодаря pub use (не через internal::)
use ex_10_05_pub_use_reexport::compute;

#[test]
fn reexported() {
    assert_eq!(compute(5), 105);
    assert_eq!(compute(0), 100);
}
