//! 04 (1x) - RwLock: реестр "ключ -> счётчик". Эталонное решение.

use std::collections::HashMap;
use std::sync::RwLock;

pub struct Registry {
    map: RwLock<HashMap<String, i64>>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            map: RwLock::new(HashMap::new()),
        }
    }

    pub fn incr(&self, key: &str, delta: i64) {
        let mut map = self.map.write().unwrap();
        *map.entry(key.to_string()).or_insert(0) += delta;
    }

    pub fn get(&self, key: &str) -> Option<i64> {
        self.map.read().unwrap().get(key).copied()
    }

    pub fn sum(&self) -> i64 {
        self.map.read().unwrap().values().sum()
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}
