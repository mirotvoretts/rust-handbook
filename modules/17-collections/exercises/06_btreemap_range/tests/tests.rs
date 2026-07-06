use ex_17_06_btreemap_range::EventLog;

fn sample() -> EventLog {
    let mut log = EventLog::new();
    log.record(300, "third");
    log.record(100, "first");
    log.record(200, "second");
    log
}

#[test]
fn range_is_chronological() {
    let log = sample();
    assert_eq!(log.between(100, 300), vec!["first", "second"]); // to исключён
    assert_eq!(log.between(0, 1000), vec!["first", "second", "third"]);
    assert_eq!(log.between(400, 500), Vec::<&str>::new());
}

#[test]
fn edges() {
    let log = sample();
    assert_eq!(log.earliest(), Some("first"));
    assert_eq!(log.latest(), Some("third"));
    assert_eq!(EventLog::new().earliest(), None);
}
