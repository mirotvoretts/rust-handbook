//! 06 (3x) — GAT: тип-заём с параметром времени жизни.
//!
//! Window<'a> — ассоциированный тип, зависящий от 'a: окно занимает из self.
//! Реализуйте трейт для Chunks (окно — срез &[i32]) и для Text (окно — &str по
//! СИМВОЛЬНЫМ границам не паримся: байтовые индексы, тест использует ASCII).

pub trait Windows {
    type Window<'a>
    where
        Self: 'a;

    /// Окно ширины size, начиная с start. None — если не помещается.
    fn window_at(&self, start: usize) -> Option<Self::Window<'_>>;
}

pub struct Chunks {
    pub data: Vec<i32>,
    pub size: usize,
}

pub struct Text {
    pub content: String,
    pub size: usize,
}

impl Windows for Chunks {
    type Window<'a> = &'a [i32];

    fn window_at(&self, start: usize) -> Option<&[i32]> {
        todo!("slice.get(start..start + self.size)")
    }
}

impl Windows for Text {
    type Window<'a> = &'a str;

    fn window_at(&self, start: usize) -> Option<&str> {
        todo!("str.get(start..start + self.size)")
    }
}
