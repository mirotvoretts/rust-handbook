//! 09 (2x) — Структура, владеющая `String`. Эталонное решение.

pub struct User {
    name: String,
    age: u32,
}

impl User {
    pub fn new(name: &str, age: u32) -> Self {
        User {
            name: name.to_string(),
            age,
        }
    }

    pub fn greet(&self) -> String {
        format!("Привет, {}!", self.name)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn birthday(&mut self) {
        self.age += 1;
    }

    pub fn age(&self) -> u32 {
        self.age
    }
}
