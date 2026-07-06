//! 03 (0x) - HashMap: базовые операции.
//!
//! insert возвращает старое значение (Option), get - Option<&V>, remove - Option<V>.
//! Обратите внимание: get принимает &K (и даже &str для ключа String - Borrow).

use std::collections::HashMap;

/// Телефонная книга.
pub struct Phonebook {
    entries: HashMap<String, String>,
}

impl Phonebook {
    pub fn new() -> Self {
        todo!()
    }

    /// Добавляет запись; возвращает СТАРЫЙ номер, если имя уже было.
    pub fn add(&mut self, name: &str, phone: &str) -> Option<String> {
        todo!("insert возвращает ровно это")
    }

    /// Ищет номер по имени.
    pub fn lookup(&self, name: &str) -> Option<&str> {
        todo!("get + as_deref (Option<&String> -> Option<&str>)")
    }

    /// Удаляет запись; true, если была.
    pub fn forget(&mut self, name: &str) -> bool {
        todo!("remove(...).is_some()")
    }

    pub fn len(&self) -> usize {
        todo!()
    }
}
