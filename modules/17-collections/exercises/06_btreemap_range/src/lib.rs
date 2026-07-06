//! 06 (1x) - BTreeMap: порядок и диапазоны.
//!
//! Журнал событий с ключом-таймстампом. BTreeMap держит ключи отсортированными:
//! range даёт срез по времени, first/last_key_value - края.

use std::collections::BTreeMap;

pub struct EventLog {
    events: BTreeMap<u64, String>,
}

impl EventLog {
    pub fn new() -> Self {
        todo!()
    }

    pub fn record(&mut self, timestamp: u64, event: &str) {
        todo!()
    }

    /// События в [from, to) в хронологическом порядке.
    pub fn between(&self, from: u64, to: u64) -> Vec<&str> {
        todo!("self.events.range(from..to)")
    }

    /// Самое раннее событие.
    pub fn earliest(&self) -> Option<&str> {
        todo!("first_key_value")
    }

    /// Самое позднее событие.
    pub fn latest(&self) -> Option<&str> {
        todo!()
    }
}
