//! 12 (3x) — Курсор-токенизатор. Эталонное решение.

pub struct Tokenizer<'a> {
    rest: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Tokenizer<'a> {
        Tokenizer { rest: text }
    }

    pub fn next_word(&mut self) -> Option<&'a str> {
        let trimmed = self.rest.trim_start();
        if trimmed.is_empty() {
            self.rest = trimmed;
            return None;
        }
        let end = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());
        let word = &trimmed[..end];
        self.rest = &trimmed[end..];
        Some(word)
    }
}
