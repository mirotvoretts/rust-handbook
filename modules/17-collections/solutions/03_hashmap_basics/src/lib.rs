//! 03 (0x) — HashMap: базовые операции. Эталонное решение.

use std::collections::HashMap;

/// Телефонная книга.
pub struct Phonebook {
    entries: HashMap<String, String>,
}

impl Phonebook {
    pub fn new() -> Self {
        Phonebook { entries: HashMap::new() }
    }

    /// Добавляет запись; возвращает СТАРЫЙ номер, если имя уже было.
    pub fn add(&mut self, name: &str, phone: &str) -> Option<String> {
        self.entries.insert(name.to_string(), phone.to_string())
    }

    /// Ищет номер по имени.
    pub fn lookup(&self, name: &str) -> Option<&str> {
        self.entries.get(name).map(|s| s.as_str())
    }

    /// Удаляет запись; true, если была.
    pub fn forget(&mut self, name: &str) -> bool {
        self.entries.remove(name).is_some()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}
