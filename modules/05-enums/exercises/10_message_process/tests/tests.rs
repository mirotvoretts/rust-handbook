use ex_05_10_message_process::{describe_move, process, Message};

#[test]
fn processing() {
    assert_eq!(process(&Message::Quit), "quit");
    assert_eq!(process(&Message::Move { x: 3, y: -1 }), "move to (3, -1)");
    assert_eq!(process(&Message::Write(String::from("hi"))), "write: hi");
    assert_eq!(process(&Message::ChangeColor(255, 0, 128)), "color 255,0,128");
}

#[test]
fn move_description() {
    assert_eq!(describe_move(&Message::Move { x: 5, y: 5 }), "diagonal");
    assert_eq!(describe_move(&Message::Move { x: 1, y: 2 }), "move");
    assert_eq!(describe_move(&Message::Quit), "other");
    assert_eq!(describe_move(&Message::Write(String::from("x"))), "other");
}
