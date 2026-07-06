//! 12 (3x) — Стек на `Vec<i32>`. Эталонное решение.

pub struct Stack {
    items: Vec<i32>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, value: i32) {
        self.items.push(value);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<i32> {
        self.items.last().copied()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
