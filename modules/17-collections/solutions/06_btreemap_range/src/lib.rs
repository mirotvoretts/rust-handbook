//! 06 (1x) - BTreeMap: порядок и диапазоны. Эталонное решение.

use std::collections::BTreeMap;

pub struct EventLog {
    events: BTreeMap<u64, String>,
}

impl Default for EventLog {
    fn default() -> Self {
        Self::new()
    }
}

impl EventLog {
    pub fn new() -> Self {
        EventLog { events: BTreeMap::new() }
    }

    pub fn record(&mut self, timestamp: u64, event: &str) {
        self.events.insert(timestamp, event.to_string());
    }

    /// События в [from, to) в хронологическом порядке.
    pub fn between(&self, from: u64, to: u64) -> Vec<&str> {
        self.events.range(from..to).map(|(_, e)| e.as_str()).collect()
    }

    /// Самое раннее событие.
    pub fn earliest(&self) -> Option<&str> {
        self.events.first_key_value().map(|(_, e)| e.as_str())
    }

    /// Самое позднее событие.
    pub fn latest(&self) -> Option<&str> {
        self.events.last_key_value().map(|(_, e)| e.as_str())
    }
}
