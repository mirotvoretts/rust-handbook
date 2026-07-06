//! 10 (2x) - Enum с разными видами данных, guard и связывание. Эталонное решение.

pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

pub fn process(msg: &Message) -> String {
    match msg {
        Message::Quit => String::from("quit"),
        Message::Move { x, y } => format!("move to ({x}, {y})"),
        Message::Write(text) => format!("write: {text}"),
        Message::ChangeColor(r, g, b) => format!("color {r},{g},{b}"),
    }
}

pub fn describe_move(msg: &Message) -> &'static str {
    match msg {
        Message::Move { x, y } if x == y => "diagonal",
        Message::Move { .. } => "move",
        _ => "other",
    }
}
