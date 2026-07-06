//! 10 (2x) — Enum с разными видами данных, guard и связывание.
//!
//! `Message` — классический enum с вариантами всех форм: без данных, структурный, кортежный.
//! Разберите его исчерпывающим `match`, извлекая поля.

pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

/// Текстовое описание сообщения:
///   Quit               -> "quit"
///   Move { x, y }      -> "move to (x, y)"      (например "move to (3, -1)")
///   Write(text)        -> "write: text"
///   ChangeColor(r,g,b) -> "color r,g,b"          (например "color 255,0,128")
pub fn process(msg: &Message) -> String {
    todo!("match msg по всем четырём вариантам")
}

/// Описывает вариант Move через guard: если x == y -> "diagonal", иначе Move -> "move";
/// любое другое сообщение -> "other".
pub fn describe_move(msg: &Message) -> &'static str {
    todo!("ветка Move с guard x == y, затем Move без guard, затем _")
}
