//! 09 (2x) — «Карающий» деструктор и panicking(). Эталонное решение.

pub struct Transaction {
    pub committed_flag: bool, // для тестов; в жизни тут был бы дескриптор
}

impl Transaction {
    pub fn begin() -> Self {
        Transaction { committed_flag: false }
    }

    /// Явное успешное завершение: Drop не должен сработать.
    pub fn commit(self) -> bool {
        std::mem::forget(self);
        true
    }

    /// Явная отмена: Drop тоже не должен сработать.
    pub fn rollback(self) -> bool {
        std::mem::forget(self);
        true
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            panic!("транзакция не завершена: вызовите commit или rollback");
        }
        // во время чужой паники — молчим: паника в панике = abort
    }
}
