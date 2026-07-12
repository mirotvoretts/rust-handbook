use ex_32_07_connection::Connection;

#[test]
fn full_flow() {
    let c = Connection::new();
    let c = c.connect("db.local");
    assert_eq!(c.address(), "db.local");
    let c = c.authenticate("alice");
    assert_eq!(c.query("SELECT 1"), "alice@db.local: SELECT 1");
    let _d = c.disconnect();
}

// Не компилируется - защита протокола:
// Connection::new().query("x");
// Connection::new().authenticate("a");
