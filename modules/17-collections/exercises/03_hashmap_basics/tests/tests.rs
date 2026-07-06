use ex_17_03_hashmap_basics::Phonebook;

#[test]
fn add_and_lookup() {
    let mut pb = Phonebook::new();
    assert_eq!(pb.add("alice", "111"), None);
    assert_eq!(pb.lookup("alice"), Some("111"));
    assert_eq!(pb.lookup("bob"), None);
}

#[test]
fn insert_returns_old() {
    let mut pb = Phonebook::new();
    pb.add("alice", "111");
    assert_eq!(pb.add("alice", "222"), Some(String::from("111")));
    assert_eq!(pb.lookup("alice"), Some("222"));
    assert_eq!(pb.len(), 1);
}

#[test]
fn forget_works() {
    let mut pb = Phonebook::new();
    pb.add("x", "1");
    assert!(pb.forget("x"));
    assert!(!pb.forget("x"));
    assert_eq!(pb.len(), 0);
}
