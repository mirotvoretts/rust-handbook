use sol_13_05_version_ord::{newest, Version};

fn ver(major: u32, minor: u32, patch: u32) -> Version {
    Version { major, minor, patch }
}

#[test]
fn ordering_rules() {
    assert!(ver(2, 0, 0) > ver(1, 9, 9));
    assert!(ver(1, 2, 0) > ver(1, 1, 9));
    assert!(ver(1, 1, 2) > ver(1, 1, 1));
    assert!(ver(1, 0, 0) == ver(1, 0, 0));
}

#[test]
fn sortable_now() {
    let mut vs = vec![ver(1, 2, 3), ver(0, 9, 0), ver(1, 0, 0)];
    vs.sort(); // sort требует Ord
    assert_eq!(vs, vec![ver(0, 9, 0), ver(1, 0, 0), ver(1, 2, 3)]);
}

#[test]
fn newest_picks_max() {
    let vs = [ver(1, 0, 0), ver(2, 1, 0), ver(2, 0, 9)];
    assert_eq!(newest(&vs), Some(ver(2, 1, 0)));
    assert_eq!(newest(&[]), None);
}
