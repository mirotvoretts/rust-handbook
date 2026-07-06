use sol_05_01_enum_basics::{clockwise, opposite, to_degrees, Direction::*};

#[test]
fn opposites() {
    assert_eq!(opposite(North), South);
    assert_eq!(opposite(East), West);
    assert_eq!(opposite(South), North);
    assert_eq!(opposite(West), East);
}

#[test]
fn rotation() {
    assert_eq!(clockwise(North), East);
    assert_eq!(clockwise(East), South);
    assert_eq!(clockwise(South), West);
    assert_eq!(clockwise(West), North);
}

#[test]
fn degrees() {
    assert_eq!(to_degrees(North), 0);
    assert_eq!(to_degrees(East), 90);
    assert_eq!(to_degrees(South), 180);
    assert_eq!(to_degrees(West), 270);
}
