//! 04 (1x) - RwLock: много читателей XOR один писатель.
//!
//! Реализуй потокобезопасный реестр "ключ -> счётчик" на RwLock<HashMap<String, i64>>.
//! read() отдаёт разделяемый доступ (много одновременно), write() - эксклюзивный.
//! Методы, которые только читают (get, sum), берут read-замок; incr меняет данные и
//! берёт write-замок. Все методы принимают &self: изменяемость - внутренняя, через
//! RwLock, поэтому Registry можно делить между потоками по общей ссылке (Arc).

use std::collections::HashMap;
use std::sync::RwLock;

pub struct Registry {
    // TODO: одно поле - RwLock<HashMap<String, i64>>.
}

impl Registry {
    /// Пустой реестр.
    pub fn new() -> Self {
        todo!()
    }

    /// Прибавить `delta` к счётчику ключа (создать со значением delta, если ключа
    /// ещё нет). Берёт write-замок.
    pub fn incr(&self, key: &str, delta: i64) {
        todo!()
    }

    /// Текущее значение счётчика ключа или None. Берёт read-замок.
    pub fn get(&self, key: &str) -> Option<i64> {
        todo!()
    }

    /// Сумма всех счётчиков. Берёт read-замок.
    pub fn sum(&self) -> i64 {
        todo!()
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}
