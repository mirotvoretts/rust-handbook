use ex_15_06_gat_windows::{Chunks, Text, Windows};

#[test]
fn chunk_windows() {
    let c = Chunks { data: vec![1, 2, 3, 4, 5], size: 3 };
    assert_eq!(c.window_at(0), Some(&[1, 2, 3][..]));
    assert_eq!(c.window_at(2), Some(&[3, 4, 5][..]));
    assert_eq!(c.window_at(3), None); // не помещается
}

#[test]
fn text_windows() {
    let t = Text { content: String::from("hello"), size: 2 };
    assert_eq!(t.window_at(0), Some("he"));
    assert_eq!(t.window_at(3), Some("lo"));
    assert_eq!(t.window_at(4), None);
}

#[test]
fn windows_are_borrows_not_copies() {
    let c = Chunks { data: vec![10, 20], size: 1 };
    let w = c.window_at(1).unwrap();
    assert_eq!(w[0], 20);
    // w - заём из c: c жива, пока жив w (NLL следит)
    assert_eq!(c.data.len(), 2);
}
